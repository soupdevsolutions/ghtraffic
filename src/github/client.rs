use crate::env::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET};
use lambda_http::tracing;

use super::{AccessTokenResponse, GithubError};

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
            .await?;
        tracing::info!("Response: {:?}", response);

        let response = response.json::<AccessTokenResponse>().await?;
        tracing::info!("Deserialized Response: {:?}", response);

        Ok(response)
    }
}

impl Default for GithubClient {
    fn default() -> Self {
        Self::new()
    }
}
