use crate::abstract_trait::PostsRepositoryTrait;
use crate::domain::{CreatePostRequest, PostRelationResponse, UpdatePostRequest};
use crate::entities::{comments, prelude::Posts, posts};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Set,
};
use tracing::{error, info};

pub struct PostRepository {
    db_pool: DatabaseConnection,
}

impl PostRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl PostsRepositoryTrait for PostRepository {
    async fn get_all_posts(&self) -> Result<Vec<posts::Model>, DbErr> {
        Posts::find().all(&self.db_pool).await
    }

    async fn get_post(&self, post_id: i32) -> Result<Option<posts::Model>, DbErr> {
        Posts::find_by_id(post_id).one(&self.db_pool).await
    }

    async fn get_post_relation(&self, post_id: i32) -> Result<Vec<PostRelationResponse>, DbErr> {
        // Log the start of the function
        info!("Fetching post relation for post ID: {}", post_id);

        match Posts::find()
            .filter(posts::Column::Id.eq(post_id))
            .find_with_related(comments::Entity)
            .all(&self.db_pool)
            .await
        {
            Ok(post_with_comments) => {
                info!(
                    "Successfully fetched post with related comments for post ID: {}",
                    post_id
                );

                let result = post_with_comments
                    .into_iter()
                    .flat_map(|(post, comments)| {
                        comments.into_iter().map(move |comment| {
                            PostRelationResponse::from_post_and_comment(&post, &comment)
                        })
                    })
                    .collect::<Vec<_>>();

            
                info!(
                    "Found {} related comments for post ID: {}",
                    result.len(),
                    post_id
                );

                Ok(result)
            }
            Err(e) => {
               
                error!(
                    "Failed to fetch post relation for post ID: {}. Error: {:?}",
                    post_id, e
                );
                Err(e)
            }
        }
    }

    async fn create_post(&self, input: &CreatePostRequest) -> Result<posts::Model, DbErr> {
        let new_post = posts::ActiveModel {
            title: Set(input.title.to_string()),
            body: Set(input.body.to_string()),
            slug: Set(input.title.to_string()),
            img: Set(input.img.to_string()),
            category_id: Set(input.category_id),
            user_id: Set(input.user_id),
            user_name: Set(input.user_name.to_string()),
            ..Default::default()
        };

        match new_post.insert(&self.db_pool).await {
            Ok(post) => Ok(post),
            Err(e) => {
                error!("Failed to create post: {:?}", e);
                Err(e)
            }
        }
    }

    async fn update_post(&self, input: &UpdatePostRequest) -> Result<posts::Model, DbErr> {
        let id = match input.post_id {
            Some(id) => id,
            None => return Err(DbErr::Custom("Post ID is required".to_string())),
        };

        let post = Posts::find_by_id(id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_owned()))?;

        let mut post: posts::ActiveModel = post.into();
        post.title = Set(input.title.to_string());
        post.body = Set(input.body.to_string());
        post.img = Set(input.img.to_string());
        post.category_id = Set(input.category_id);
        post.user_id = Set(input.user_id);
        post.user_name = Set(input.user_name.to_string());

        post.update(&self.db_pool).await
    }

    async fn delete_post(&self, post_id: i32) -> Result<(), DbErr> {
        let post = Posts::find_by_id(post_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Post not found".to_owned()))?;

        post.delete(&self.db_pool).await?;
        Ok(())
    }
}
