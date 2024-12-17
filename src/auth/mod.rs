use serde::Serialize;

pub mod jwt;

#[derive(Debug, Serialize)]
pub struct UserToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
}
