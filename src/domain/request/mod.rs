mod category;
mod post;
mod comment;
mod user;
mod auth;

pub use self::category::{CreateCategoryRequest, UpdateCategoryRequest};
pub use self::post::{
    CreatePostRequest,
    UpdatePostRequest
};


pub use self::comment::{
    CreateCommentRequest,
    UpdateCommentRequest
};

pub use self::auth::{
    LoginRequest,
    RegisterRequest
};

pub use self::user::{
    CreateUserRequest,
    UpdateUserRequest
};