use crate::{
    schema::guild::GuildSchema,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    response::IntoResponse,
};
use reqwest::StatusCode;

use serde::Deserialize;
use tracing::{debug, error, warn};

// ! FOR GUILDS 0 if FALSE, 1 if TRUE
// ! This is used to enable or disable features for a guild

#[worker::send]
#[axum::debug_handler]
pub async fn get(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, StatusCode> {
    let guild: Vec<GuildSchema> = match database.execute(GuildSchema::get_by_id(&guild_id)).await {
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

#[derive(Debug, Deserialize)]
pub struct NewGuildQuery {
    shard_id: u32,
}

#[worker::send]
#[axum::debug_handler]
pub async fn create(
    Path(guild_id): Path<String>,
    Query(params): Query<NewGuildQuery>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    debug!("Creating new guild with ID: {}", guild_id);
    requested_user.bot_protection("Create Guild")?;

    let shard_id = params.shard_id;

    let _: Vec<()> = (database
        .execute(GuildSchema::insert(&guild_id, shard_id))
        .await)
        .map_err(|e| {
            error!("Failed to create guild entry: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create guild entry".to_string(),
            )
        })?;

    debug!("Guild created successfully with ID: {}", guild_id);
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct GuildQuery {
    unavailable: Option<bool>,
}

#[worker::send]
#[axum::debug_handler]
pub async fn disable(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Query(params): Query<GuildQuery>,
) -> Result<(), (StatusCode, String)> {
    requested_user.bot_protection("Disable Guild")?;
    let unavailable = params.unavailable.unwrap_or(false); // Default to false if not provided, don't want to accidentally delete guilds if the parameter is missing
    if unavailable {
        let delete_query = GuildSchema::delete(&guild_id);
        let _guilds: Vec<GuildSchema> = (database.execute(delete_query).await).map_err(|e| {
            warn!("Failed to get guild for deletion: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete guild entry".to_string(),
            )
        })?;
    } else {
        let disable_query = GuildSchema::disable(&guild_id);
        if let Err(e) = database.execute(disable_query).await {
            warn!("Failed to disable guild entry: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to disable guild entry".to_string(),
            ));
        }
    }
    Ok(())
}
