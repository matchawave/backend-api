use axum::{
    body::Body,
    extract::{Path, Request},
    http::Response,
    Extension,
};
use reqwest::{header::USER_AGENT, StatusCode};
use tracing::error;
use worker::Env;

use crate::state::user::RequestedUser;

#[worker::send]
pub async fn handle_websocket(
    Path(id): Path<String>,
    Extension(env): Extension<Env>,
    Extension(requested_user): Extension<RequestedUser>,
    req: Request,
) -> Result<Response<Body>, StatusCode> {
    let Ok(bot_token) = env.secret("DISCORD_BOT_TOKEN").map(|s| s.to_string()) else {
        error!("Failed to get bot token");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let url = req.uri().clone();
    let Ok(mut new_req) = worker::Request::new(&url.to_string(), worker::Method::Get) else {
        error!("Failed to create new request");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let headers = new_req.headers_mut().expect("Failed to get headers");
    for (key, value) in req.headers().iter() {
        if let Err(e) = headers.append(key.as_str(), value.to_str().expect("Invalid header value"))
        {
            error!("Failed to append header {}: {}", key, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    match requested_user {
        RequestedUser::UserWithToken(user) => {
            // The id will be the guild ID
            if let Err(err) =
                headers.set(USER_AGENT.as_str(), format!("DiscordGuild/{}", id).as_str())
            {
                error!("Failed to set header {}: {}", USER_AGENT, err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        RequestedUser::Bot(bot) => {
            if bot.token() != bot_token {
                error!("Bot token mismatch");
                return Err(StatusCode::UNAUTHORIZED);
            }
            if let Err(err) = headers.set(USER_AGENT.as_str(), "DiscordBot") {
                error!("Failed to set header {}: {}", USER_AGENT, err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        RequestedUser::User => {
            error!("User requested without a bot or guild ID");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    let Ok(object) = env.durable_object("BOTROOM") else {
        error!("Failed to get durable object");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(object_id) = object.id_from_name(&bot_token) else {
        error!("Failed to get object ID");
        return Err(StatusCode::NOT_FOUND);
    };

    let Ok(stub) = object_id.get_stub() else {
        error!("Failed to get object stub");
        return Err(StatusCode::NOT_FOUND);
    };

    let Ok(res) = stub.fetch_with_request(new_req).await else {
        error!("Failed to fetch durable object");
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(res.into())
}
