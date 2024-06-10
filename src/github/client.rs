use super::{AccessTokenResponse, GithubError};

pub struct GithubClient {
    client: reqwest::Client,
}

impl GithubClient {
    pub fn new() -> Self {
        GithubClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn exchange_code(&self, code: impl Into<String>) -> Result<AccessTokenResponse, GithubError> {
        let client_id = std::env::var("GITHUB_CLIENT_ID")?;
        let client_secret = std::env::var("GITHUB_CLIENT_SECRET")?;

        let url = String::from("https://github.com/login/oauth/access_token");
        let params = [
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code.into()),
        ];

        let response = self.client
            .post(url)
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        Ok(response)
    }
}
