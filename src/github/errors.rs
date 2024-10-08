use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Failed to parse repository: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Failed to read environment variable: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Failed to send request: {0}")]
    RequestError(#[from] reqwest::Error),
}
