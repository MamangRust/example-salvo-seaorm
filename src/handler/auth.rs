use crate::{
    config::Claims,
    domain::{ApiResponse, LoginRequest, RegisterRequest, UserResponse},
    middleware::jwt_auth,
    state::AppState,
};
use salvo::{oapi::extract::JsonBody, prelude::*, size_limiter};
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
#[handler]
pub async fn register_user_handler(req: JsonBody<RegisterRequest>, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();

    let body = req.into_inner();

    match state.di_container.auth_service.register_user(&body).await {
        Ok(response) => {
            res.status_code(StatusCode::OK);
            res.render(Json(json!({
                "status": "success",
                "message": "User registered successfully",
                "data": response
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(json!({
                "status": "fail",
                "message": "Registration failed",
                "error": e.to_string()
            })));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<String>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "auth"
)]
#[handler]
pub async fn login_user_handler(req: JsonBody<LoginRequest>, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();

    let body = req.into_inner();

    match state.di_container.auth_service.login_user(&body).await {
        Ok(response) => {
            res.status_code(StatusCode::OK).render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED).render(Json(e));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Get Me user", body = ApiResponse<UserResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "auth",
)]
#[handler]
pub async fn get_user_handler(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();

    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let jwt_data = match depot.jwt_auth_data::<Claims>() {
                Some(data) => data,
                None => {
                    res.status_code(StatusCode::UNAUTHORIZED)
                        .render(Json(json!({
                            "status": "fail",
                            "message": "Invalid JWT token"
                        })));
                    return;
                }
            };

            match state
                .di_container
                .user_service
                .find_by_id(jwt_data.claims.user_id as i32)
                .await
            {
                Ok(Some(user)) => {
                    res.status_code(StatusCode::OK).render(Json(json!({
                        "status": "success",
                        "message": "User fetched successfully",
                        "data": { "user": user }
                    })));
                }
                Ok(None) => {
                    res.status_code(StatusCode::NOT_FOUND).render(Json(json!({
                        "status": "fail",
                        "message": "User not found"
                    })));
                }
                Err(_) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
                        .render(Json(json!({
                            "status": "error",
                            "message": "Could not fetch user"
                        })));
                }
            }
        }
        JwtAuthState::Unauthorized => {
            res.status_code(StatusCode::UNAUTHORIZED)
                .render(Json(json!({
                    "status": "error",
                    "message": "Could not fetch user"
                })));
        }
        JwtAuthState::Forbidden => {
            res.status_code(StatusCode::FORBIDDEN).render(Json(json!({
                "status": "error",
                "message": "Could not fetch user"
            })));
        }
    }
}

pub fn auth_routes() -> Router {
    let public_routes = Router::new()
        .push(Router::with_path("api/auth/register").post(register_user_handler))
        .push(Router::with_path("api/auth/login").post(login_user_handler));
     

    let private_routes = Router::new().push(
        Router::with_path("api/users/me")
            .hoop(jwt_auth())
            .get(get_user_handler)
    );

    return Router::new()
        .push(private_routes)
        .push(public_routes)
        .hoop(size_limiter::max_size(1024 * 16));
}
