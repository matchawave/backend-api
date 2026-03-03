use axum::{extract::Request, middleware::Next, response::Response, Extension};
use reqwest::StatusCode;
use worker::Env;

use crate::state::{database::Database, user::RequestedUser};

pub async fn middleware(
    Extension(env): Extension<Env>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    request.extensions_mut().insert(Database::new(&env, "DB"));
    let response = next.run(request).await;
    Ok(response)
}
