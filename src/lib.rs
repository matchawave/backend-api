use axum::{
    body::Body,
    http::{method, HeaderValue, Response},
    Extension, Router,
};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method, StatusCode,
};
use tower_http::cors::{AllowCredentials, CorsLayer};
use tower_service::Service;
use tracing::warn;
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

/**
 * Helper function to copy an axum::http::Request to a worker::Request, optionally changing the path
 * This is used when forwarding requests from axum to worker, since axum's Request and worker::Request are different types
 * If new_path is Some, the path of the new request will be set to new_path, otherwise it will be the same as the original request
 * This function also copies headers from the original request to the new request, and converts the HTTP method from axum's Method to worker's Method
 */
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

/**
 * Copies headers from an axum::http::HeaderMap to a worker::Headers
 * This is used when copying requests from axum to worker, since axum's Request and worker::Request use different Header types
 * Note that this does not copy all headers, only the ones that are valid in worker::Headers, and it will skip any invalid headers
 */
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

/**
 * Converts an axum::http::Method to a worker::Method
 * Defaults to GET for unsupported methods
 * This is used when copying requests from axum to worker, since axum's Request and worker::Request use different Method types
 */
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

/**
 * Checks if a string is a valid Discord snowflake
 * A snowflake is a string of 17-19 digits
 */
pub fn check_snowflake(id: &str) -> bool {
    let snowflake_regex =
        regex::Regex::new(r"^\d{17,19}$").expect("Failed to compile snowflake regex");
    snowflake_regex.is_match(id)
}

/**
 * Checks if a string is a valid Discord snowflake, and returns an error response if it is not
 * This is used in API endpoints that take Discord IDs as parameters, to validate the input before processing the request
 */
#[macro_export]
macro_rules! snowflake_protection {
    ($id:expr) => {
        if !$crate::check_snowflake(&$id) {
            tracing::warn!("Invalid {}: {}", stringify!($id), $id);
            return Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "Invalid ID provided for {}: must be a valid Discord snowflake",
                    stringify!($id)
                ),
            ));
        }
    };
}
