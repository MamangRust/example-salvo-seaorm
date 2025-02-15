mod auth;
mod category;
mod comment;
mod posts;
mod user;

use std::sync::Arc;

use crate::state::AppState;
use salvo::prelude::*;
use salvo::http::header::{self, HeaderValue};
use salvo::http::response::ResBody;
use utoipa::openapi::security::SecurityScheme;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::Config;

pub use self::auth::auth_routes;
pub use self::category::category_routes;
pub use self::comment::comment_routes;
pub use self::posts::post_routes;
pub use self::user::user_routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login_user_handler, 
        auth::get_user_handler, 
        auth::register_user_handler,
        user::create_user,
        user::find_user_by_email,
        user::update_user,
        user::delete_user,
        category::get_categories,
        category::get_category,
        category::create_category,
        category::update_category,
        category::delete_category,
        comment::get_comments,
        comment::get_comment,
        comment::create_comment,
        comment::update_comment,
        comment::delete_comment,
        posts::get_posts,
        posts::get_post,
        posts::get_post_relation,
        posts::create_post,
        posts::update_post,
        posts::delete_post,
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication endpoints."),
        (name = "Categories", description = "Categories management endpoints."),
        (name = "Posts", description = "Posts management endpoints."),
        (name = "Comments", description = "Comments management endpoints."),
        (name = "Users", description = "Users management endpoints.")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

pub struct AppRouter;

impl AppRouter {
    pub async fn serve(port: u16, app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
        let config = Arc::new(Config::from("/api-doc/openapi.json"));

        let router = Router::new()
            .hoop(affix_state::inject(app_state.clone()))
            .push(auth_routes())
            .push(category_routes())
            .push(comment_routes())
            .push(post_routes())
            .push(user_routes())
            .push(Router::with_path("/api-doc/openapi.json").get(openapi_json))
            .push(
                Router::with_path("/swagger-ui/{**}")
                    .hoop(affix_state::inject(config))
                    .get(serve_swagger),
            );

        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::new(&addr).bind().await;
        println!("Server running on http://{}", listener.local_addr()?);

        Server::new(listener).serve(router).await;

        Ok(())
    }
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("Hello");
}

#[handler]
pub async fn openapi_json(res: &mut Response) {
    res.render(Json(ApiDoc::openapi()))
}

#[handler]
pub async fn serve_swagger(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let config = depot.obtain::<Arc<Config>>().unwrap();
    let path = req.uri().path();

    let tail = match path.strip_prefix("/swagger-ui/") {
        Some(tail) => tail,
        None => {
            res.status_code(StatusCode::NOT_FOUND);
            return;
        }
    };

    match utoipa_swagger_ui::serve(tail, config.clone()) {
        Ok(swagger_file) => {
            if let Some(file) = swagger_file {
                res.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(&file.content_type).unwrap(),
                );
                res.body(ResBody::Once(file.bytes.to_vec().into()));
            } else {
                res.status_code(StatusCode::NOT_FOUND);
            }
        }
        Err(_error) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
