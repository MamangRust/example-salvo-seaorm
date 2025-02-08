use crate::{abstract_trait::{CommentServiceTrait, DynCommentRepository, }, domain::{ApiResponse, CommentResponse, CreateCommentRequest, ErrorResponse, UpdateCommentRequest},  utils::AppError};
use async_trait::async_trait;

pub struct CommentService {
    repository: DynCommentRepository,
}

impl CommentService {
    pub fn new(repository: DynCommentRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommentServiceTrait for CommentService {
    async fn get_comments(&self) -> Result<ApiResponse<Vec<CommentResponse>>, ErrorResponse> {
        let comments = self.repository.find_all().await .map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        let response = comments.into_iter().map(|comment| {
            CommentResponse::from(comment)
        }).collect();
        
        Ok(ApiResponse{
            status: "success".to_string(),
            message: "Comments retrieved successfully".to_string(),
            data: response
        })
    }

    async fn get_comment(&self, id: i32) -> Result<Option<ApiResponse<CommentResponse>>, ErrorResponse> {
        let comment = self.repository.find_by_id(id).await .map_err(AppError::from).map_err(ErrorResponse::from)?;

        
        
        if let Some(comment) = comment{
            Ok(Some(ApiResponse{
                status: "success".to_string(),
                message: "Comment retrieved successfully".to_string(),
                data: CommentResponse::from(comment),
            }))
        }else{
            Err(ErrorResponse::from(AppError::NotFound(format!("Comment with id {} not found", id))))
        }
    }

    async fn create_comment(&self, input: &CreateCommentRequest) -> Result<ApiResponse<CommentResponse>, ErrorResponse> {
        let comment = self.repository.create(input).await .map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Comment created successfully".to_string(),
            data: CommentResponse::from(comment),
        })
    }

    async fn update_comment(&self, input: &UpdateCommentRequest) -> Result<Option<ApiResponse<CommentResponse>>, ErrorResponse> {
        let comment = self.repository.update(input).await.map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "Comment updated successfully".to_string(),
            data: CommentResponse::from(comment),
        }))
    }

    async fn delete_comment(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        self.repository.delete(id).await.map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Comment deleted successfully".to_string(),
            data: (),
        })
    }
}
