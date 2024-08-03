use crate::env::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET};
use lambda_http::tracing::{self, warn};

use super::{AccessTokenResponse, GithubError, UserRepository, UserRepositoryViews};

const GITHUB_AUTH_BASE_URI: &str = "https://github.com";
const GITHUB_API_BASE_URI: &str = "https://api.github.com";

pub enum GithubClientBaseUri {
    Default,
    Custom(String),
}

#[derive(Debug)]
pub struct GithubClient {
    client: reqwest::Client,
    auth_base_uri: String,
    api_base_uri: String,
}

impl GithubClient {
    pub fn new(auth_base_uri: GithubClientBaseUri, api_base_uri: GithubClientBaseUri) -> Self {
        let auth_base_uri = match auth_base_uri {
            GithubClientBaseUri::Default => GITHUB_AUTH_BASE_URI.to_owned(),
            GithubClientBaseUri::Custom(value) => value,
        };

        let api_base_uri = match api_base_uri {
            GithubClientBaseUri::Default => GITHUB_API_BASE_URI.to_owned(),
            GithubClientBaseUri::Custom(value) => value,
        };

        GithubClient {
            client: reqwest::Client::new(),
            auth_base_uri,
            api_base_uri,
        }
    }

    #[tracing::instrument]
    pub fn get_login_uri(&self) -> String {
        "https://github.com/apps/ghtraffic/installations/select_target".to_string()
    }

    #[tracing::instrument]
    pub async fn exchange_code(&self, code: String) -> Result<AccessTokenResponse, GithubError> {
        let client_id = std::env::var(GITHUB_CLIENT_ID)?;
        let client_secret = std::env::var(GITHUB_CLIENT_SECRET)?;

        let url = format!("{}/login/oauth/access_token", self.auth_base_uri);
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
        let url = format!("{}/user/repos", self.api_base_uri);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "ghtraffic")
            .query(&[("per_page", 100)])
            .query(&[("visibility", "public")])
            .query(&[("sort", "pushed")])
            .send()
            .await?;

        let response = response.json::<Vec<UserRepository>>().await?;
        Ok(response)
    }

    #[tracing::instrument]
    pub async fn get_repository_traffic(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<UserRepositoryViews>, GithubError> {
        let url = format!(
            "{}/repos/{}/{}/traffic/popular/referrers",
            self.api_base_uri, owner, repo
        );

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "ghtraffic")
            .send()
            .await?;

        let response = response.json::<Vec<UserRepositoryViews>>().await?;
        Ok(response)
    }
}

impl Default for GithubClient {
    fn default() -> Self {
        GithubClient::new(GithubClientBaseUri::Default, GithubClientBaseUri::Default)
    }
}
