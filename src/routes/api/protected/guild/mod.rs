use std::collections::HashMap;

use crate::{
    schema::{Guild, GuildSchema},
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};
use axum::{
    extract::{Path, Query},
    http::Response,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_query::{Expr, OnConflict, SqliteQueryBuilder};
use tracing::{error, warn};

mod settings;
// ! FOR GUILDS 0 if FALSE, 1 if TRUE
// ! This is used to enable or disable features for a guild

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_guild).post(create_new_guild).delete(disable_guild),
        )
        .nest("/settings", settings::router())
}

#[axum::debug_handler]
#[worker::send]
async fn get_guild(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, StatusCode> {
    let guild: Vec<GuildSchema> = match database
        .execute(GuildSchema::get_by_id(guild_id.clone()))
        .await
    {
        Ok(guild) => guild,
        Err(e) => {
            warn!("Failed to get guild: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    if guild.is_empty() {
        warn!("Guild with ID {} not found", guild_id);
        return Ok(StatusCode::OK.into_response());
    }
    if guild.len() > 1 {
        error!("Multiple guilds found with ID {}", guild_id);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok((StatusCode::OK, Json(guild[0].to_owned())).into_response())
}

#[worker::send]
async fn create_new_guild(
    Path(guild_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    if !requested_user.is_bot() {
        warn!("Non-bot user attempted to create guild entry");
        return Err((
            StatusCode::FORBIDDEN,
            "Only registered bots can create guild entries".to_string(),
        ));
    }
    let shard_id = if let Some(shard_str) = params.get("shard_id") {
        shard_str.parse::<u32>().map_err(|_| {
            warn!("Invalid shard_id parameter: {}", shard_str);
            (
                StatusCode::BAD_REQUEST,
                "Invalid shard_id parameter".to_string(),
            )
        })?
    } else {
        warn!("shard_id parameter is missing");
        return Err((
            StatusCode::BAD_REQUEST,
            "shard_id parameter is missing".to_string(),
        ));
    };

    if let Err(e) = database
        .execute(GuildSchema::insert(guild_id, shard_id))
        .await
    {
        error!("Failed to create guild entry: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create guild entry".to_string(),
        ));
    }
    Ok(())
}

#[worker::send]
async fn disable_guild(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    if !requested_user.is_bot() {
        warn!("Non-bot user attempted to disable guild entry");
        return Err((
            StatusCode::FORBIDDEN,
            "Only registered bots can disable guild entries".to_string(),
        ));
    }

    if let Err(e) = database.execute(GuildSchema::disable(guild_id)).await {
        warn!("Failed to disable guild entry: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to disable guild entry".to_string(),
        ));
    }
    Ok(())
}

#[worker::send]
pub async fn delete_guild(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    if !requested_user.is_bot() {
        warn!("Non-bot user attempted to delete guild entry");
        return Err((
            StatusCode::FORBIDDEN,
            "Only registered bots can delete guild entries".to_string(),
        ));
    }
    let _guilds: Vec<GuildSchema> = (database
        .execute(GuildSchema::get_by_id(guild_id.clone()))
        .await)
        .map_err(|e| {
            warn!("Failed to get guild for deletion: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete guild entry".to_string(),
            )
        })?;
    Ok(())
}
