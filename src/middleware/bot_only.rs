use axum::{Extension, extract::Request, middleware::Next, response::Response};
use reqwest::StatusCode;

use crate::state::user::RequestedUser;

pub async fn middleware(
    Extension(requested_user): Extension<RequestedUser>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !requested_user.is_bot() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let response = next.run(request).await;
    Ok(response)
}
