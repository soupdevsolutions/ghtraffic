use askama::Template;
use ghtraffic::{
    github::GithubClient,
    requests::get_cookie,
    templates::{RepoListTemplate, WelcomeTemplate},
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    
    let token = match get_cookie(&event, "token") {
        Some(token) => token,
        None => return Ok(Response::builder().status(302).header("location", "/").body(Body::Empty)?),
    };

    let repositories = github_client.get_user_repositories(token).await?;

    let template = RepoListTemplate { repositories };
    let data = template.render()?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(data.into())
        .map_err(Box::new)?;
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
