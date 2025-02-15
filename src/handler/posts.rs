use salvo::prelude::*;
use serde_json::json;
use crate::{
    domain::{ApiResponse, CreatePostRequest, PostRelationResponse, PostResponse, UpdatePostRequest}, middleware::jwt_auth, state::AppState
};


#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "Get list of posts", body = ApiResponse<Vec<PostResponse>>)
    ),
    tag = "Posts"
)]
#[handler] 
pub async fn get_posts(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    match state.di_container.post_service.get_all_posts().await {
        Ok(posts) => res.render(Json(posts)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Get post by ID", body = ApiResponse<PostResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "Posts"
)]
#[handler]
pub async fn get_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let post_id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.post_service.get_post(post_id).await {
        Ok(Some(post)) => res.render(Json(post)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                "status": "fail",
                "message": "Post not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}/relation",
    responses(
        (status = 200, description = "Get related posts", body = ApiResponse<Vec<PostRelationResponse>>),
        (status = 404, description = "Post not found")
    ),
    tag = "Posts"
)]
#[handler]
pub async fn get_post_relation(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let post_id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.post_service.get_post_relation(post_id).await {
        Ok(posts) => res.render(Json(posts)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = ApiResponse<PostResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Posts"
)]
#[handler]
pub async fn create_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let body = match req.parse_body::<CreatePostRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));

            return;
        }
    };

    match state.di_container.post_service.create_post(&body).await {
        Ok(post) => {
            res.status_code(StatusCode::CREATED).render(Json({
                json!({
                    "status": "success",
                    "message": "Post created successfully",
                    "data": post,
                })
            }));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated successfully", body = ApiResponse<PostResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 5000, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Posts"
)]
#[handler]
pub async fn update_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let post_id: i32 = req.param("id").unwrap_or_default();
    let mut body = match req.parse_body::<UpdatePostRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    body.post_id = Some(post_id);

    match state.di_container.post_service.update_post(&body).await {
        Ok(post) => res.render(Json(post)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Posts"
)]
#[handler]
pub async fn delete_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let post_id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.post_service.delete_post(post_id).await {
        Ok(_) => {
            res.status_code(StatusCode::OK).render(Json(json!({
                "status": "success",
                "message": "Post deleted successfully"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

pub fn post_routes() -> Router {
    let protected_routes = Router::new()
        .push(Router::with_path("api/posts").post(create_post))
        .push(Router::with_path("api/posts/{id}").put(update_post))
        .push(Router::with_path("api/posts/{id}").delete(delete_post))
        
        .hoop(jwt_auth());

        let public_routes = Router::new()
        .push(Router::with_path("api/posts").get(get_posts))
        .push(Router::with_path("api/posts/{id}").get(get_post))
        .push(Router::with_path("api/posts/{id}/relation").get(get_post_relation));
    

   return Router::new()
        .push(protected_routes)
        .push(public_routes);
}
