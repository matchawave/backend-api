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
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
struct NewGuildQuery {
    shard_id: u32,
}

#[worker::send]
async fn create_new_guild(
    Path(guild_id): Path<String>,
    Query(params): Query<NewGuildQuery>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    requested_user.bot_protection("Create Guild")?;

    let shard_id = params.shard_id;

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
    requested_user.bot_protection("Disable Guild")?;

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
    requested_user.bot_protection("Delete Guild")?;
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
