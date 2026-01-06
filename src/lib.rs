use axum::{
    body::Body,
    http::{HeaderValue, Response},
    Extension, Router,
};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tower_http::cors::{AllowCredentials, CorsLayer};
use tower_service::Service;
use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::{event, Context, Env, HttpRequest, Result};

use crate::state::{
    database::{Database, Databases},
    server_info::ServerInfo,
};
pub mod durables;
pub mod middleware;
pub mod schema;
pub mod services;
pub mod state;

mod routes;

pub const DISCORD_API_BASE_URL: &str = "https://discord.com/api/v10";
pub const DASHBOARD_URL: &str = "http://localhost:5173";

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);

    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

fn cors_layer(webpage: &str) -> CorsLayer {
    let webpage_header = HeaderValue::from_str(webpage).expect("Invalid URL for CORS");
    CorsLayer::new()
        .allow_origin(webpage_header)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(AllowCredentials::yes())
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, ctx: Context) -> Result<Response<Body>> {
    console_error_panic_hook::set_once();

    let databases = Databases {
        general: Database::new(env.d1("DB")?),
        levels: Database::new(env.d1("LEVELS_DB")?),
    };

    let server_info = ServerInfo::new(&env)?;

    let mut app = Router::new()
        .merge(routes::router())
        .layer(Extension(databases))
        .layer(Extension(env))
        .layer(cors_layer(server_info.webpage()))
        .layer(Extension(server_info));

    Ok(app.call(req).await?)
}
