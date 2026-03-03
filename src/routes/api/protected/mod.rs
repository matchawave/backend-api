mod afk;
mod birthday;
mod gateway;
mod guild;

use axum::{routing::get, Router};

use crate::middleware;

pub fn router() -> Router {
    Router::new()
        .nest("/guild/{guild_id}", guild::router())
        .nest("/afk", afk::router())
        .nest("/birthday", birthday::router())
        .route("/gateway/{bot_id}", get(gateway::handle_websocket))
        .layer(axum::middleware::from_fn(
            middleware::api_protect::middleware,
        ))
}
