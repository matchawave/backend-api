use axum::{Router, routing::get};

mod afk;
mod birthday;

pub fn router() -> Router {
    Router::new()
        .route("/afk", get(afk::get).post(afk::set).delete(afk::remove))
        .route("/afk/config", get(afk::get_config).post(afk::set_config))
        .route("/birthday", get(birthday::get).post(birthday::set))
}
