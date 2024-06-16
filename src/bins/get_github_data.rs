use askama::Template;
use ghtraffic::{
    github::GithubClient,
    templates::{AuthenticatedTemplate, WelcomeTemplate},
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

pub async fn render_authenticated_page(
    github_client: &GithubClient,
    code: String,
) -> anyhow::Result<String> {
    let token = github_client.exchange_code(code).await?;

    let template = AuthenticatedTemplate {
        token: token.access_token,
    };
    Ok(template.render()?)
}

#[tracing::instrument]
pub fn render_welcome_page(github_client: &GithubClient) -> anyhow::Result<String> {
    let login_uri = github_client.get_login_uri()?;

    let template = WelcomeTemplate { login_uri };
    Ok(template.render()?)
}

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::info!("Received event: {:?}", event);
    let code = event
        .query_string_parameters()
        .first("code")
        .map(|v| v.to_string());

    let data = match code {
        Some(code) => render_authenticated_page(github_client, code).await?,
        None => render_welcome_page(github_client)?,
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
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
