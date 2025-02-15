use salvo::prelude::*;
use serde_json::json;
use crate::{
    
    domain::{ApiResponse, CommentResponse, CreateCommentRequest, UpdateCommentRequest}, middleware::jwt_auth, state::AppState
};

#[utoipa::path(
    get,
    path = "/api/comments",
    responses(
        (status = 200, description = "Get all comments", body = ApiResponse<Vec<CommentResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
#[handler]
pub async fn get_comments(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    match state.di_container.comment_service.get_comments().await {
        Ok(comments) => res.render(Json(comments)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!({
                "status": "error",
                "message": "Failed to fetch comments",
                "error": format!("{:?}", e)
            })));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/comments/{id}",
    responses(
        (status = 200, description = "Get a comment", body = ApiResponse<CommentResponse>),
        (status = 404, description = "Comment not found")
    ),
    params(
        ("id" = i32, Path, description = "Comment ID")
    ),
    tag = "Comments"
)]
#[handler]
pub async fn get_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let comment_id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.comment_service.get_comment(comment_id).await {
        Ok(Some(comment)) => res.render(Json(comment)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                "status": "fail",
                "message": "Comment not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/comments",
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment created", body = ApiResponse<CommentResponse>),
        (status = 400, description = "Invalid request body")
    ),
    tag = "Comments"
)]
#[handler]
pub async fn create_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let body = match req.parse_body::<CreateCommentRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    match state.di_container.comment_service.create_comment(&body).await {
        Ok(comment) => {
            res.status_code(StatusCode::CREATED).render(Json(comment));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}


#[utoipa::path(
    put,
    path = "/api/comments/{id}",
    request_body = UpdateCommentRequest,
    responses(
        (status = 200, description = "Comment updated", body = ApiResponse<CommentResponse>),
        (status = 404, description = "Comment not found")
    ),
    params(
        ("id" = i32, Path, description = "Comment ID")
    ),
    tag = "Comments"
)]
#[handler]
pub async fn update_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let body = match req.parse_body::<UpdateCommentRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    match state.di_container.comment_service.update_comment(&body).await {
        Ok(Some(comment)) => res.render(Json(comment)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                "status": "fail",
                "message": "Comment not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/comments/{id}",
    responses(
        (status = 200, description = "Comment deleted successfully", body=Value),
        (status = 500, description = "Failed to delete comment")
    ),
    params(
        ("id" = i32, Path, description = "Comment ID")
    ),
    tag = "Comments"
)]
#[handler] 
pub async fn delete_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let comment_id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.comment_service.delete_comment(comment_id).await {
        Ok(_) => {
            res.status_code(StatusCode::OK);
            res.render(Json(json!({
                "status": "success",
                "message": "Comment deleted successfully"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

pub fn comment_routes() -> Router {
    let protected_routes = Router::new()
        .push(Router::with_path("api/comments").get(get_comments))
        .push(Router::with_path("api/comments/{id}").get(get_comment))
        .push(Router::with_path("api/comments").post(create_comment))
        .push(Router::with_path("api/comments/{id}").put(update_comment))
        .push(Router::with_path("api/comments/{id}").delete(delete_comment))
        .hoop(jwt_auth());

    return Router::new()
        .push(protected_routes);
}
