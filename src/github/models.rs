use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}
