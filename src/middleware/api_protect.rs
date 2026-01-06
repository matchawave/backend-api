use axum::{extract::Request, middleware::Next, response::Response, Extension};
use reqwest::StatusCode;

use crate::state::user::RequestedUser;

pub async fn middleware(
    Extension(requested_user): Extension<RequestedUser>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let RequestedUser::User = requested_user {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let response = next.run(request).await;

    Ok(response)
}
