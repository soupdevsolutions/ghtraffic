use askama::Template;
use ghtraffic::{
    github::GithubClient, requests::create_set_cookie_header, templates::IndexTemplate,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::info!("Received event: {:?}", event);
    let code = event
        .query_string_parameters()
        .first("code")
        .map(String::from);

    let template = IndexTemplate { code: code.clone() };

    let mut token = None;
    if let Some(code) = code {
        token = Some(github_client.exchange_code(code).await?.access_token);
    }

    let data = template.render()?;
    let mut resp = Response::builder()
        .status(200)
        .header("content-type", "text/html");

    if let Some(token) = token {
        let cookie_header = create_set_cookie_header("token", &token, 3600 * 24 * 7);
        resp = resp.header("Set-Cookie", cookie_header);
    }
    let resp = resp.body(data.into())?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let github_client = GithubClient::default();
    run(service_fn(|request| handler(&github_client, request))).await
}
