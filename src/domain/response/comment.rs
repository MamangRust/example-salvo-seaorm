use utoipa::ToSchema;
use serde::{Deserialize, Serialize};


use crate::entities::comments;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CommentResponse {
    pub id: i32,
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

impl From<comments::Model> for CommentResponse {
    fn from(comment: comments::Model) -> Self {
        CommentResponse {
            id: comment.id,
            id_post_comment: comment.id_post_comment,
            user_name_comment: comment.user_name_comment,
            comment: comment.comment,
        }
    }
}