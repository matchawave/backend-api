use axum::{body::Body, response::Response, routing::get, Router};

use crate::middleware;

pub mod api;
pub mod cdn;

pub fn router() -> Router {
    Router::new()
        .nest("/api", api::router())
        .nest("/cdn", cdn::router())
        .route("/", get(root))
        .fallback(fallback)
        .layer(axum::middleware::from_fn(
            middleware::requested_user::middleware,
        ))
}
async fn fallback() -> Response<Body> {
    Response::new(Body::from("Not Found"))
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}
