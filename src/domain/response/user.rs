use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::entities::users;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        UserResponse {
            id: user.id,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
        }
    }
}