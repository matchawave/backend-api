use axum::{Router, routing::get};

mod birthday;
mod configuration;
mod info;
mod member;
mod settings;
mod shard;

pub(super) use settings::SettingsBody;

pub fn router() -> Router {
    Router::new()
        .route("/", get(info::get).post(info::create).delete(info::disable))
        .route("/birthday", get(birthday::upcoming))
        .route("/shard", get(shard::get))
        .nest("/member", member::router())
        .nest("/settings", settings::router())
}
