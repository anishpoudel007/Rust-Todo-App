use serde::Serialize;

use crate::models::_entities::{task, user, user_profile};

#[derive(Debug, Serialize)]
pub struct UserSerializer {
    pub name: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserWithProfileSerializer {
    pub name: String,
    pub username: String,
    pub email: String,
    pub profile: Option<user_profile::Model>,
}

impl From<user::Model> for UserSerializer {
    fn from(value: user::Model) -> Self {
        Self {
            name: value.name,
            username: value.username,
            email: value.email,
        }
    }
}

impl From<(user::Model, Option<user_profile::Model>)> for UserWithProfileSerializer {
    fn from(value: (user::Model, Option<user_profile::Model>)) -> Self {
        let (user, profile) = value;

        Self {
            name: user.name,
            username: user.username,
            email: user.email,
            profile,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TaskSerializer {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub date_created: chrono::naive::NaiveDateTime,
    pub date_updated: Option<String>,
}

impl From<task::Model> for TaskSerializer {
    fn from(value: task::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status,
            date_created: value.date_created,
            date_updated: value.date_updated,
        }
    }
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
