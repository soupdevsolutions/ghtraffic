use askama::Template;
use ghtraffic::{github::GithubClient, templates::IndexTemplate};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::info!("Received event: {:?}", event);
    let code = event
        .query_string_parameters()
        .first("code")
        .map(String::from);

    let template = IndexTemplate { code: code.clone() };

    let mut token = String::new();
    if let Some(code) = code {
        token = github_client.exchange_code(code).await?.access_token;
    }

    let data = template.render()?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .header("Set-Cookie", format!("token={}", token))
        .body(data.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let github_client = GithubClient::new();
    run(service_fn(|request| handler(&github_client, request))).await
}
