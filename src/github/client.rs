use crate::env::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET};
use lambda_http::tracing::{self, warn};

use super::{AccessTokenResponse, GithubError, UserRepository, UserRepositoryViews};

#[derive(Debug)]
pub struct GithubClient {
    client: reqwest::Client,
}

impl GithubClient {
    pub fn new() -> Self {
        GithubClient {
            client: reqwest::Client::new(),
        }
    }

    #[tracing::instrument]
    pub fn get_login_uri(&self) -> Result<String, GithubError> {
        let client_id = std::env::var(GITHUB_CLIENT_ID)?;

        let url = format!("https://github.com/login/oauth/authorize?client_id={client_id}");
        Ok(url)
    }

    #[tracing::instrument]
    pub async fn exchange_code(&self, code: String) -> Result<AccessTokenResponse, GithubError> {
        let client_id = std::env::var(GITHUB_CLIENT_ID)?;
        let client_secret = std::env::var(GITHUB_CLIENT_SECRET)?;

        let url = String::from("https://github.com/login/oauth/access_token");
        let params = [
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
        ];

        let response = self
            .client
            .post(url)
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        Ok(response)
    }

    #[tracing::instrument(skip(token))]
    pub async fn get_user_repositories(
        &self,
        token: String,
    ) -> Result<Vec<UserRepository>, GithubError> {
        let url = String::from("https://api.github.com/user/repos");

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "ghtraffic")
            .query(&[("per_page", 100)])
            .query(&[("visibility", "public")])
            .send()
            .await?;
        tracing::info!("Response: {:?}", response);

        let mut response = response.json::<Vec<UserRepository>>().await?;
        response.sort_by_key(|repo| repo.owner.login.clone());
        Ok(response)
    }

    #[tracing::instrument]
    pub async fn get_repository_traffic(
        &self,
        token: String,
        owner: String,
        repo: String,
    ) -> Result<UserRepositoryViews, GithubError> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}/traffic/views");

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "ghtraffic")
            .send()
            .await?;

        let response = response.json::<UserRepositoryViews>().await?;
        Ok(response)
    }
}

impl Default for GithubClient {
    fn default() -> Self {
        Self::new()
    }
}
