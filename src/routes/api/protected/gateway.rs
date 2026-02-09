use axum::{
    body::Body,
    extract::{Path, Request},
    http::Response,
    Extension,
};
use reqwest::{header::USER_AGENT, StatusCode};
use tracing::{debug, error, info};
use worker::Env;

use crate::{
    durables::{bot::BotDurable, DurableFetch},
    state::user::RequestedUser,
};

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

    if let RequestedUser::Bot(bot) = &requested_user {
        if bot.token() != bot_token {
            error!("Bot token mismatch");
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        error!("Requested user is not a bot");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut new_req = crate::copy_request(&req, None).map_err(|e| {
        error!("Failed to copy request: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Err(err) = new_req
        .headers_mut()
        .expect("Failed to get headers")
        .set(USER_AGENT.as_str(), "DiscordBot")
    {
        error!("Failed to set header {}: {}", USER_AGENT, err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let stub = BotDurable::fetch_object(&env, &bot_token).map_err(|e| {
        error!("Failed to fetch bot durable object: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = stub.fetch_with_request(new_req).await.map_err(|e| {
        error!("Failed to fetch from durable object: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(response.into())
}
