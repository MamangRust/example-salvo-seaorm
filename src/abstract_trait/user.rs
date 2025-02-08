use sea_orm::DbErr;
use std::sync::Arc;

use async_trait::async_trait;

use crate::{domain::{ApiResponse, CreateUserRequest, ErrorResponse, UpdateUserRequest, UserResponse}, entities::users};

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, DbErr>;
    async fn create_user(
        &self,
        input: &CreateUserRequest
    ) -> Result<users::Model, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, DbErr>;
    async fn update_user(
        &self,
        input: &UpdateUserRequest
    ) -> Result<users::Model, DbErr>;
    async fn delete_user(&self, email: &str) -> Result<(), DbErr>;
}

#[async_trait]
pub trait UserServiceTrait {
    async fn create_user(
        &self,
        input: &CreateUserRequest
    ) -> Result<ApiResponse<UserResponse>, ErrorResponse>;
    async fn find_by_email_exists(&self, email: &str) -> Result<ApiResponse<bool>, ErrorResponse>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse>;
    async fn find_by_id(&self, id: i32) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse>;
    async fn update_user(
        &self,
        input: &UpdateUserRequest
    ) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse>;
    async fn delete_user(&self, email: &str) -> Result<ApiResponse<()>, ErrorResponse>;
}