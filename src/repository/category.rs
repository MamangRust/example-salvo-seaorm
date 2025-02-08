use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,  Set};

use crate::domain::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::entities::{categories, prelude::Categories,};
use crate::abstract_trait::CategoryRepositoryTrait;



pub struct CategoryRepository{
    db_pool: DatabaseConnection
}


impl CategoryRepository{
    pub fn new(db_pool: DatabaseConnection) -> Self{
        Self { db_pool }
    }
}

#[async_trait]
impl CategoryRepositoryTrait for CategoryRepository {
    async fn find_all(&self) -> Result<Vec<categories::Model>, DbErr> {
        Categories::find()
            .all(&self.db_pool)
            .await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<categories::Model>, DbErr> {
        Categories::find_by_id(id)
            .one(&self.db_pool)
            .await
    }

    async fn create(&self, input: &CreateCategoryRequest) -> Result<categories::Model, DbErr> {
        let category = categories::ActiveModel {
            name: Set(input.name.clone()),
            ..Default::default() 
        };

        category.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateCategoryRequest) -> Result<categories::Model, DbErr> {
        let id = match input.id {
            Some(id) => id, 
            None => return Err(DbErr::Custom("Category ID is required".to_string())), 
        };

        let mut category: categories::ActiveModel = Categories::find_by_id(id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("Category not found".to_string()))?
            .into();

        if let Some(name) = &input.name {
            category.name = Set(name.clone());
        }

        category.update(&self.db_pool).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        let category: categories::ActiveModel = Categories::find_by_id(id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("Category not found".to_string()))?
            .into();

        category.delete(&self.db_pool).await.map(|_| ())
    }
}
