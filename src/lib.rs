use axum::{
    body::Body,
    http::{method, HeaderValue, Response},
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
use worker::{event, Context, Env, HttpRequest, Request, Result};

use crate::state::{database::Database, server_info::ServerInfo};
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
    let database: Database = env.d1("DB")?.into();
    let server_info = ServerInfo::new(&env)?;

    let mut app = Router::new()
        .merge(routes::router())
        .layer(Extension(database))
        .layer(Extension(env))
        .layer(cors_layer(server_info.webpage()))
        .layer(Extension(server_info));

    Ok(app.call(req).await?)
}

/// Helper macro to count the number of expressions at compile time
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

pub fn copy_request(
    req: &axum::http::Request<Body>,
    new_path: Option<&str>,
) -> worker::Result<worker::Request> {
    let new_url = if let Some(new_path) = new_path {
        let mut url = worker::Url::parse(&req.uri().to_string())?;
        url.set_path(new_path);
        url.to_string()
    } else {
        req.uri().to_string()
    };
    let mut new_req = worker::Request::new(&new_url, convert_method(req.method()))?;
    copy_headers(req.headers(), new_req.headers_mut()?)?;
    Ok(new_req)
}

pub fn copy_headers(from: &axum::http::HeaderMap, to: &mut worker::Headers) -> worker::Result<()> {
    for (key, value) in from.into_iter() {
        let key = key.as_str();
        let value = value.to_str().map_err(|e| {
            worker::Error::RustError(format!("Invalid header value for {}: {}", key, e))
        })?;
        to.append(key, value)?;
    }
    Ok(())
}

pub fn convert_method(method: &axum::http::Method) -> worker::Method {
    match *method {
        axum::http::Method::GET => worker::Method::Get,
        axum::http::Method::POST => worker::Method::Post,
        axum::http::Method::PUT => worker::Method::Put,
        axum::http::Method::DELETE => worker::Method::Delete,
        axum::http::Method::HEAD => worker::Method::Head,
        axum::http::Method::OPTIONS => worker::Method::Options,
        axum::http::Method::CONNECT => worker::Method::Connect,
        axum::http::Method::TRACE => worker::Method::Trace,
        _ => worker::Method::Get, // Default to GET for unsupported methods
    }
}
