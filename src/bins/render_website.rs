use askama::Template;
use ghtraffic::{
    github::GithubClient, requests::create_set_cookie_header, requests::get_cookie,
    requests::TOKEN_COOKIE, templates::IndexTemplate,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::debug!("Received event: {:?}", event);

    let mut token = get_cookie(&event, TOKEN_COOKIE);
    if token.is_none() {
        let code = event
            .query_string_parameters()
            .first("code")
            .map(String::from);
        if let Some(code) = code {
            token = Some(github_client.exchange_code(code).await?.access_token);
        }
    }

    let template = IndexTemplate {
        authenticated: token.is_some(),
        login_uri: github_client.get_login_uri().ok(),
    };
    let data = template.render()?;
    let mut resp = Response::builder()
        .status(200)
        .header("content-type", "text/html");

    if let Some(token) = token {
        let cookie_header = create_set_cookie_header("token", &token, 3600 * 2);
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
