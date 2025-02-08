use utoipa::ToSchema;
use serde::Serialize;

use crate::entities::{comments, posts};

#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

impl From<posts::Model> for PostResponse {
    fn from(post: posts::Model) -> Self {
        PostResponse {
            id: post.id,
            title: post.title,
            body: post.body,
            category_id: post.category_id,
            user_id: post.user_id,
            user_name: post.user_name,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostRelationResponse {
    pub post_id: i32,
    pub title: String,
    pub comment_id: i32,
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

impl PostRelationResponse {
    pub fn from_post_and_comment(post: &posts::Model, comment: &comments::Model) -> Self {
        PostRelationResponse {
            post_id: post.id,
            title: post.title.clone(),
            comment_id: comment.id,
            id_post_comment: comment.id_post_comment,
            user_name_comment: comment.user_name_comment.clone(),
            comment: comment.comment.clone(),
        }
    }
}
