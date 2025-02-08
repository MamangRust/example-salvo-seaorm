use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,  Set};

use crate::domain::{CreateCommentRequest, UpdateCommentRequest};
use crate::entities::{comments, prelude::Comments};
use crate::abstract_trait::CommentRepositoryTrait;

pub struct CommentRepository {
    db_pool: DatabaseConnection,
}

impl CommentRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CommentRepositoryTrait for CommentRepository {
    async fn find_all(&self) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .all(&self.db_pool)
            .await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<comments::Model>, DbErr> {
        Comments::find_by_id(id)
            .one(&self.db_pool)
            .await
    }

    async fn create(&self, input: &CreateCommentRequest) -> Result<comments::Model, DbErr> {
        let comment = comments::ActiveModel {
            id_post_comment: Set(input.id_post_comment),
            user_name_comment: Set(input.user_name_comment.clone()),
            comment: Set(input.comment.clone()),
            ..Default::default()
        };

        comment.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateCommentRequest) -> Result<comments::Model, DbErr> {
        let mut comment: comments::ActiveModel = Comments::find_by_id(input.id_post_comment)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("Comment not found".to_string()))?
            .into();

        comment.user_name_comment = Set(input.user_name_comment.clone());
        comment.comment = Set(input.comment.clone());

        comment.update(&self.db_pool).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        let comment: comments::ActiveModel = Comments::find_by_id(id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("Comment not found".to_string()))?
            .into();

        comment.delete(&self.db_pool).await.map(|_| ())
    }
}
