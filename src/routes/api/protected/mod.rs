mod afk;
mod gateway;
mod guild;

use axum::{
    routing::{delete, get},
    Router,
};

use crate::middleware;

pub fn router() -> Router {
    Router::new()
        .nest("/guild/{guild_id}", guild::router())
        .nest("/afk", afk::router())
        .route("/guilds/{guild_id}", delete(guild::delete_guild))
        .route("/gateway/{bot_id}", get(gateway::handle_websocket))
        .layer(axum::middleware::from_fn(
            middleware::api_protect::middleware,
        ))
}
