use std::collections::HashMap;

use askama::Template;
use ghtraffic::{
    github::{GithubClient, Repository, UserAggregatedViews},
    requests::get_cookie,
    templates::RepoViewsTemplate,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
pub async fn render_repos_views(
    github_client: &GithubClient,
    token: String,
    repos: Vec<Repository>,
) -> anyhow::Result<String> {
    let mut referrers = HashMap::new();
    let mut total_count = 0;
    let mut total_uniques = 0;

    for repo in repos {
        let referrers_views = github_client
            .get_repository_traffic(&token, &repo.owner, &repo.name)
            .await?;
        tracing::info!("Calculating views for {}", repo.name);

        for referrer in referrers_views {
            let mut entry = *referrers.entry(referrer.referrer.clone()).or_insert((0, 0));
            entry.0 += referrer.count;
            entry.1 += referrer.uniques;

            referrers.insert(referrer.referrer, entry);

            total_count += referrer.count;
            total_uniques += referrer.uniques;
        }
    }

    let mut referrers: Vec<_> = referrers.into_iter().collect();
    referrers.sort_by(|a, b| a.0.cmp(&b.0));

    let template = RepoViewsTemplate {
        views: UserAggregatedViews {
            total_count,
            total_uniques,
            referrers,
        },
    };

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

    let query_string_parameters = event.query_string_parameters();
    let repo_names = query_string_parameters.all("repo_name");
    tracing::info!("Query string parameters: {:?}", query_string_parameters);

    if repo_names.is_none() {
        return Ok(Response::builder()
            .status(400)
            .body("repo_name query parameter is required.".into())
            .map_err(Box::new)?);
    }

    let mut repos = vec![];
    for repo_name in repo_names.unwrap() {
        let repo = Repository::parse(repo_name)?;
        repos.push(repo);
    }
    tracing::info!("Repo names: {:?}", repos);

    let data = render_repos_views(github_client, token, repos).await?;
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

#[cfg(test)]
mod tests {

    use super::handler;
    use ghtraffic::github::{GithubClient, GithubClientBaseUri};
    use lambda_http::http::Request;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_calculate_traffic_returns_error_when_token_is_not_present() {
        let mock_server = MockServer::start().await;
        let github_client = GithubClient::new(
            GithubClientBaseUri::Custom(mock_server.uri()),
            GithubClientBaseUri::Custom(mock_server.uri()),
        );

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let event = Request::get("/").body(lambda_http::Body::Empty).unwrap();

        let response = handler(&github_client, event).await.unwrap();
        assert_eq!(response.status(), 401);
    }
}
