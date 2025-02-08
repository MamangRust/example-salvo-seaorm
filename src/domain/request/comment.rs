use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateCommentRequest {
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize,ToSchema)]
pub struct UpdateCommentRequest {
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}