use salvo::prelude::*;
use serde_json::json;
use crate::{
    domain::{ApiResponse, CreateUserRequest, UpdateUserRequest, UserResponse}, middleware::jwt_auth, state::AppState
};

#[utoipa::path(
    post,
    path = "/api/user",
    responses(
        (status = 200, description = "Create user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
)]
#[handler]
pub async fn create_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let body = match req.parse_body::<CreateUserRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    match state.di_container.user_service.create_user(&body).await {
        Ok(response) => {
            res.status_code(StatusCode::CREATED).render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/user/{email}",
    responses(
        (status = 200, description = "Find Email user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
)]
#[handler]
pub async fn find_user_by_email(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let email: String = req.param("email").unwrap_or_default();

    match state.di_container.user_service.find_user_by_email(&email).await {
        Ok(Some(response)) => res.render(Json(response)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                "status": "fail",
                "message": "User not found"
            })));
        }
        Err(_e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!({
                 "status": "fail",
                "message": "Internal server error"
            })));
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/user/{id}",
    responses(
        (status = 200, description = "Update user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
)]
#[handler]
pub async fn update_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();
    let mut body = match req.parse_body::<UpdateUserRequest>().await {
        Ok(body) => body,
        Err(_) => {
           
            res.render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    body.id = Some(id);

    match state.di_container.user_service.update_user(&body).await {
        Ok(Some(response)) => res.render(Json(response)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                "status": "fail",
                "message": "User not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/user/{email}",
    responses(
        (status = 200, description = "User ", body = Value),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
)]
#[handler]
pub async fn delete_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let email: String = req.param("email").unwrap_or_default();

    match state.di_container.user_service.delete_user(&email).await {
        Ok(_) => {
            res.status_code(StatusCode::OK).render(Json(json!({
                "status": "success",
                "message": "User deleted successfully"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(e));
        }
    }
}

pub fn user_routes() -> Router {
    let protected_routes = Router::new()
        .push(Router::with_path("api/user").post(create_user))
        .push(Router::with_path("api/user/email/{email}").get(find_user_by_email))
        .push(Router::with_path("api/user/id/{id}").put(update_user))
        .push(Router::with_path("api/user/{email}").delete(delete_user))
        .hoop(jwt_auth());

    return Router::new()
        .push(protected_routes);
}
