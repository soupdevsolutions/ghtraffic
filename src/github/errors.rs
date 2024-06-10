use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Failed to read environment variable: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Failed to process request: {0}")]
    RequestError(#[from] reqwest::Error),
}
