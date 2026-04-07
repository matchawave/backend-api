mod gateway;
mod guild;
mod streams;
mod user;

use axum::{Router, middleware, routing::get};

use crate::middleware as ware;

pub fn router() -> Router {
    Router::new()
        .nest("/guild/{guild_id}", guild::router())
        .nest("/user/{user_id}", user::router())
        .nest("/stream", streams::router())
        .route("/gateway/{bot_id}", get(gateway::handle_websocket))
        .layer(middleware::from_fn(ware::api_protect::middleware))
}
