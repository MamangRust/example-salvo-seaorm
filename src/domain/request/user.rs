use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
   
    pub email: Option<String>, 
    pub password: Option<String>,
}

