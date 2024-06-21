use askama::Template;
use ghtraffic::{github::GithubClient, requests::get_cookie, templates::RepoViewsTemplate};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
pub async fn render_repo_views(
    github_client: &GithubClient,
    token: String,
    owner: String,
    repo: String,
) -> anyhow::Result<String> {
    let views = github_client
        .get_repository_traffic(token, owner, repo)
        .await?;

    let template = RepoViewsTemplate { views };

    Ok(template.render().unwrap())
}

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::info!("Received event: {:?}", event);

    let token = match get_cookie(&event, "token") {
        Some(token) => token,
        None => {
            return Ok(Response::builder()
                .status(401)
                .body("Unauthorized".into())
                .map_err(Box::new)?);
        }
    };
    tracing::info!("Token: {}", token);

    let query_string_parameters = event.query_string_parameters();
    let owner = query_string_parameters
        .first("owner")
        .map(|v| v.to_string());
    let repo_name = query_string_parameters.first("repo").map(|v| v.to_string());
    if owner.is_none() || repo_name.is_none() {
        return Ok(Response::builder()
            .status(400)
            .body("owner and repo query parameters are required.".into())
            .map_err(Box::new)?);
    }
    tracing::info!("Owner: {}, Repo: {}", owner.as_ref().unwrap(), repo_name.as_ref().unwrap());

    let data = render_repo_views(github_client, token, owner.unwrap(), repo_name.unwrap()).await?;

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
