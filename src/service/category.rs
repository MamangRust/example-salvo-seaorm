use crate::{abstract_trait::{CategoryServiceTrait, DynCategoryRepository}, domain::{ApiResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest, ErrorResponse}, utils::AppError};
use async_trait::async_trait;

pub struct CategoryService {
    repository: DynCategoryRepository
}

impl CategoryService {
    pub fn new(repository: DynCategoryRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    async fn get_categories(&self) -> Result<ApiResponse<Vec<CategoryResponse>>, ErrorResponse> {
        let categories = self.repository.find_all().await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        let category_responses: Vec<CategoryResponse> = categories.into_iter().map(|category| CategoryResponse::from(category)).collect();
    
       
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Categories retrieved successfully".to_string(),
            data: category_responses,
        })
    }

    async fn get_category(&self, id: i32) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse> {
        let category = self.repository.find_by_id(id).await.map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        if let Some(category) = category {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "Category retrieved successfully".to_string(),
                data: CategoryResponse::from(category),
            }))
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!("Category with id {} not found", id))))
        }
    }

    async fn create_category(&self, input: &CreateCategoryRequest) -> Result<ApiResponse<CategoryResponse>, ErrorResponse> {
        let category = self.repository.create(input).await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category created successfully".to_string(),
            data: CategoryResponse::from(category),
        })
    }

    async fn update_category(&self, input: &UpdateCategoryRequest) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse> {
        let category = self.repository.update(input).await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "Category updated successfully".to_string(),
            data: CategoryResponse::from(category),
        }))
    }

    async fn delete_category(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        self.repository.delete(id).await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category deleted successfully".to_string(),
            data: (),
        })
    }
}
