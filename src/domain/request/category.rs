use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize, Deserialize, Clone,Debug, ToSchema)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone,Debug, ToSchema)]
pub struct UpdateCategoryRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
}