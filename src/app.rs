use axum::{extract::DefaultBodyLimit, routing::get, Router};

use crate::routes;

pub async fn create_app() -> Router {
    Router::new()
        .route("/health_check", get(root))
        .merge(routes::image::create_route())
        .merge(routes::user::create_route())
        .merge(routes::file::create_route())
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
}

async fn root() -> &'static str {
    "Healthy!"
}
