use salvo::prelude::*;
use serde_json::json;
use std::sync::Arc;
use crate::{
    domain::{ApiResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest}, middleware::jwt_auth, state::AppState
};


#[utoipa::path(
    get,
    path = "/api/categories",
    responses(
        (status = 200, description = "List all category successfully", body = ApiResponse<Vec<CategoryResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
#[handler]
pub async fn get_categories(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<AppState>>().unwrap();
    match state.di_container.category_service.get_categories().await {
        Ok(categories) => res.render(Json(categories)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "List all category successfully", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
#[handler]
pub async fn get_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<AppState>>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();
    
    match state.di_container.category_service.get_category(id).await {
        Ok(Some(category)) => res.render(Json(category)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(json!({"status": "fail", "message": "Category not found"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/categories",
    responses(
        (status = 200, description = "Create category", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
#[handler]
pub async fn create_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<AppState>>().unwrap();
    let body = match req.parse_body::<CreateCategoryRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    match state.di_container.category_service.create_category(&body).await {
        Ok(category) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(category));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "Delete category", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
#[handler]
pub async fn update_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<AppState>>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();

    let mut body = match req.parse_body::<UpdateCategoryRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };
    
    body.id = Some(id);

    match state.di_container.category_service.update_category(&body).await {
        Ok(Some(category)) => res.render(Json(category)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(json!({"status": "fail", "message": "Category not found"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "Delete category", body = Value)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
#[handler]
pub async fn delete_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<AppState>>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.category_service.delete_category(id).await {
        Ok(_) => {
            res.status_code(StatusCode::OK);
            res.render(Json(json!({"status": "success", "message": "Category deleted successfully"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"status": "error", "message": e.to_string()})));
        }
    }
}

pub fn category_routes() -> Router {
    let protected_routes = Router::new()
        .push(Router::with_path("api/categories/{id}").get(get_category))
        .push(Router::with_path("api/categories").post(create_category))
        .push(Router::with_path("api/categories/{id}").put(update_category))
        .push(Router::with_path("api/categories/{id}").delete(delete_category))
        .hoop(jwt_auth());
        

    let public_routes = Router::new()
        .push(Router::with_path("api/categories").get(get_categories));

    return Router::new()
        .push(protected_routes)
        .push(public_routes);
}
