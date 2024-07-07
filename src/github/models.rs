use core::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::RepositoryError;

#[derive(Debug)]
pub struct Repository {
    pub full_name: String,
    pub owner: String,
    pub name: String,
}

impl Repository {
    pub fn parse(full_name: impl Into<String>) -> Result<Self, RepositoryError> {
        let full_name = full_name.into();
        let parts: Vec<&str> = full_name.split('/').collect();

        if parts.len() != 2 {
            return Err(RepositoryError::ParseError(full_name));
        }

        let owner = parts[0].to_string();
        let name = parts[1].to_string();

        Ok(Repository {
            full_name,
            owner,
            name,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRepositoryOwner {
    pub login: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRepositoryViews {
    pub referrer: String,
    pub count: u32,
    pub uniques: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserAggregatedViews {
    pub total_count: u32,
    pub total_uniques: u32,
    pub referrers: Vec<(String, (u32, u32))>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRepository {
    pub id: u64,
    pub full_name: String,
    pub name: String,
    pub owner: UserRepositoryOwner,
}

impl Display for UserRepositoryOwner {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.login)
    }
}
