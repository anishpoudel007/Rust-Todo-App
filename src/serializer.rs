use serde::Serialize;

use crate::models::_entities::{user, user_profile};

#[derive(Debug, Serialize)]
pub struct UserWithProfileSerializer {
    pub name: String,
    pub username: String,
    pub email: String,
    pub profile: Option<user_profile::Model>,
}

impl UserWithProfileSerializer {
    pub fn new(
        name: String,
        username: String,
        email: String,
        profile: Option<user_profile::Model>,
    ) -> Self {
        Self {
            name,
            username,
            email,
            profile,
        }
    }

    pub fn from_user_and_profile_model(
        user: user::Model,
        user_profile: Option<user_profile::Model>,
    ) -> Self {
        Self {
            name: user.name,
            username: user.username,
            email: user.email,
            profile: user_profile,
        }
    }
}
