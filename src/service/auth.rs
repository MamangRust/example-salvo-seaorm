use async_trait::async_trait;
use crate::{
    abstract_trait::{AuthServiceTrait, DynUserRepository},
    config::{Hashing, JwtConfig},
    domain::{ApiResponse, CreateUserRequest, ErrorResponse, LoginRequest, RegisterRequest, UserResponse},
    utils::AppError,
};

pub struct AuthService {
    repository: DynUserRepository,
    hashing: Hashing,
    jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(repository: DynUserRepository, hashing: Hashing, jwt_config: JwtConfig) -> Self {
        Self { repository, hashing, jwt_config }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register_user(&self, input: &RegisterRequest) -> Result<ApiResponse<UserResponse>, ErrorResponse> {
        let exists = self.repository.find_by_email_exists(&input.email).await
            .map_err(AppError::from)  
            .map_err(ErrorResponse::from)?; 

        if exists {
            return Err(ErrorResponse::from(AppError::EmailAlreadyExists));
        }

        let hashed_password = self.hashing.hash_password(&input.password).await
            .map_err(|e| ErrorResponse::from(AppError::HashingError(e)))?;

        let request = CreateUserRequest {
            firstname: input.firstname.clone(),
            lastname: input.lastname.clone(),
            email: input.email.clone(),
            password: hashed_password,
        };

        let create_user = self.repository.create_user(&request).await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: UserResponse::from(create_user),
        })
    }

    async fn login_user(&self, input: &LoginRequest) -> Result<ApiResponse<String>, ErrorResponse> {
        let user = self.repository.find_by_email(&input.email).await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .ok_or_else(|| ErrorResponse::from(AppError::NotFound("User not found".to_string())))?;

        if self.hashing.compare_password(&user.password, &input.password).await.is_err() {
            return Err(ErrorResponse::from(AppError::InvalidCredentials));
        }

        let token = self.jwt_config.generate_token(user.id as i64)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Login successful".to_string(),
            data: token,
        })
    }

    fn verify_token(&self, token: &str) -> Result<i64, AppError> {
        return self.jwt_config.verify_token(token).map_err(AppError::from);
    }
}
