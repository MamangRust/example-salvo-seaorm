use salvo::prelude::*;
use serde_json::json;
use crate::{
    domain::{ApiResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest}, middleware::jwt_auth, state::AppState
};


#[utoipa::path(
    get,
    path = "/api/categories",
    tag = "Categories",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Successfully retrieved list of categories", body = ApiResponse<Vec<CategoryResponse>>),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[handler]
pub async fn get_categories(depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    match state.di_container.category_service.get_categories().await {
        Ok(categories) => res.render(Json(categories)),
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    tag = "Categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Successfully retrieved category details", body = ApiResponse<CategoryResponse>),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[handler]
pub async fn get_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();
    
    match state.di_container.category_service.get_category(id).await {
        Ok(Some(category)) => res.render(Json(category)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({"status": "fail", "message": "Category not found"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/categories",
    tag = "Categories",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateCategoryRequest,
    responses(
        (status = 200, description = "Category created successfully", body = ApiResponse<CategoryResponse>),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[handler]
pub async fn create_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let body = match req.parse_body::<CreateCategoryRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };

    match state.di_container.category_service.create_category(&body).await {
        Ok(category) => {
            res.status_code(StatusCode::CREATED).render(Json(category));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    tag = "Categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    request_body = UpdateCategoryRequest,
    responses(
        (status = 200, description = "Category updated successfully", body = ApiResponse<CategoryResponse>),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[handler]
pub async fn update_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();

    let mut body = match req.parse_body::<UpdateCategoryRequest>().await {
        Ok(body) => body,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST).render(Json(json!({"status": "fail", "message": "Invalid request body"})));
            return;
        }
    };
    
    body.id = Some(id);

    match state.di_container.category_service.update_category(&body).await {
        Ok(Some(category)) => res.render(Json(category)),
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND).render(Json(json!({"status": "fail", "message": "Category not found"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!(e)));
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    tag = "Categories",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category deleted successfully", body = Value),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[handler]
pub async fn delete_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<AppState>().unwrap();
    let id: i32 = req.param("id").unwrap_or_default();

    match state.di_container.category_service.delete_category(id).await {
        Ok(_) => {
            res.status_code(StatusCode::OK).render(Json(json!({"status": "success", "message": "Category deleted successfully"})));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render(Json(json!({"status": "error", "message": e.to_string()})));
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
