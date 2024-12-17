use sea_orm::{DeriveIntoActiveModel, Set};
use serde::Deserialize;
use validator::Validate;

use crate::models::_entities::role::ActiveModel;

#[derive(Debug, Deserialize, Validate, Clone, DeriveIntoActiveModel)]
pub struct CreateRoleRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub name: String,
}

impl From<CreateRoleRequest> for ActiveModel {
    fn from(value: CreateRoleRequest) -> Self {
        Self {
            name: Set(value.name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Validate, Clone, DeriveIntoActiveModel)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub name: String,
}
