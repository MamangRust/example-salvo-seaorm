mod category;
mod post;
mod comment;
mod user;
mod auth;

pub use self::category::{
    CategoryRepositoryTrait, CategoryServiceTrait, DynCategoryRepository, DynCategoryService,
};

pub use self::post::{
    PostsRepositoryTrait, PostsServiceTrait, DynPostsRepository, DynPostsService
};

pub use self::comment::{
    CommentRepositoryTrait, CommentServiceTrait, DynCommentRepository, DynCommentService
};


pub use self::user::{
    UserRepositoryTrait, UserServiceTrait, DynUserRepository,DynUserService
};


pub use self::auth::{
    DynAuthService,
    AuthServiceTrait
};