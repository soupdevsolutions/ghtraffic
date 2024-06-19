use serde::{Deserialize, Serialize};

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
    pub count: u32,
    pub uniques: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRepository {
    pub id: u64,
    pub full_name: String,
    pub owner: UserRepositoryOwner,
}
