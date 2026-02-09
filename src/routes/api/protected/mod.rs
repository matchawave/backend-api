mod gateway;
mod guild;

use axum::{
    routing::{delete, get},
    Router,
};

use crate::middleware;

pub fn router() -> Router {
    Router::new()
        .nest("/guild/{id}", guild::router())
        .route("/guilds/{id}", delete(guild::delete_guild))
        .route("/gateway/{id}", get(gateway::handle_websocket))
        .layer(axum::middleware::from_fn(
            middleware::api_protect::middleware,
        ))
}
