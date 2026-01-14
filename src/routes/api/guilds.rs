use std::collections::HashMap;

use axum::{
    debug_handler, extract::Query, response::Redirect, routing::get, Extension, Json, Router,
};
use reqwest::StatusCode;
use tracing::{error, info, warn};
use worker::Env;

use crate::{
    services::{
        auth::{DiscordOAuth2, DiscordOAuth2Scope},
        guilds::{DiscordGuildHTTP, PartialDiscordGuild},
    },
    state::{server_info::ServerInfoArc, user::RequestedUser},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_guilds))
        .route("/mutual", get(get_mutual_guilds))
        .route("/add", get(add_guild))
        .route("/add/callback", get(add_guild_callback))
}

async fn get_guilds() -> &'static str {
    "List of guilds"
}

#[debug_handler]
#[worker::send]
async fn get_mutual_guilds(
    Extension(env): Extension<Env>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<Json<Vec<PartialDiscordGuild>>, (StatusCode, String)> {
    let Ok(bot_token) = env.secret("DISCORD_BOT_TOKEN").map(|s| s.to_string()) else {
        error!("Failed to get bot token from environment");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "".into()));
    };

    let RequestedUser::UserWithToken(user) = requested_user else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Must be authenticated to access mutual guilds".into(),
        ));
    };

    let bot_auth = format!("Bot {}", bot_token);
    let user_auth = format!("Bearer {}", user.access_token());

    let bot_client = DiscordGuildHTTP::new(bot_auth);
    let user_client = DiscordGuildHTTP::new(user_auth);

    let mutual_guilds = match bot_client.get_mutual_guilds(user_client).await {
        Ok(guilds) => guilds,
        Err(e) => {
            error!("Failed to fetch mutual guilds: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch mutual guilds".into(),
            ));
        }
    };

    Ok(Json(mutual_guilds))
}

async fn add_guild(
    Extension(env): Extension<Env>,
    Extension(server_info): Extension<ServerInfoArc>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<Redirect, StatusCode> {
    let Ok(client_id) = env.var("DISCORD_CLIENT_ID").map(|s| s.to_string()) else {
        error!("Failed to get client ID from environment");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let redirect_uri = format!("{}/api/guilds/add/callback", server_info.api_host());
    let oauth = DiscordOAuth2 {
        client_id,
        redirect_uri,
        scopes: vec![
            DiscordOAuth2Scope::Bot,
            DiscordOAuth2Scope::ApplicationsCommands,
        ],
    };
    if let RequestedUser::Bot(_) = requested_user {
        error!("Unauthorized access to add guild endpoint");
        return Err(StatusCode::UNAUTHORIZED);
    }
    info!("Redirecting to Discord OAuth2 add bot URL");
    Ok(Redirect::to(oauth.get_add_bot_url().as_str()))
}

async fn add_guild_callback(
    Extension(server_info): Extension<ServerInfoArc>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Redirect, StatusCode> {
    let webpage = server_info.webpage();
    // Check if there was an error in the OAuth flow
    if let Some(error) = params.get("error") {
        warn!("Discord OAuth error: {}", error);
        // Redirect to dashboard root on error
        return Ok(Redirect::to(&format!("{}/dashboard", webpage)));
    }

    // Get the guild_id from Discord's response
    let Some(guild_id) = params.get("guild_id") else {
        warn!("No guild_id provided in Discord callback");
        // Redirect to dashboard root if no guild_id
        return Ok(Redirect::to(&format!("{}/dashboard", webpage)));
    };

    // Redirect to the specific guild dashboard
    let dashboard_url = format!("{}/dashboard/{}", webpage, guild_id);
    info!("Redirecting to guild dashboard: {}", dashboard_url);
    Ok(Redirect::to(&dashboard_url))
}
