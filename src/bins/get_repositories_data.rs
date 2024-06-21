use askama::Template;
use ghtraffic::{
    github::GithubClient, requests::get_cookie, templates::{AuthenticatedTemplate, WelcomeTemplate}
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

#[tracing::instrument]
pub async fn render_authenticated_page(
    github_client: &GithubClient,
    token: String,
) -> anyhow::Result<String> {
    let repositories = github_client.get_user_repositories(token).await?;

    let template = AuthenticatedTemplate { repositories };
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
    let data = match get_cookie(&event, "token"){
        Some(token) => {
            render_authenticated_page(github_client, token).await?
        }
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
