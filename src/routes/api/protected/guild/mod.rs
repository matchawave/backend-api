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
            get(get_guild).post(create_new_guild).delete(delete_guild),
        )
        .nest("/settings", settings::router())
}

#[axum::debug_handler]
#[worker::send]
async fn get_guild(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<impl IntoResponse, StatusCode> {
    let guild: Vec<GuildSchema> = database
        .execute(GuildSchema::get_by_id(guild_id.clone()))
        .await?;
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
) -> Result<(), StatusCode> {
    if let RequestedUser::Bot(_) = requested_user {
        let shard_id = if let Some(shard_str) = params.get("shard_id") {
            shard_str.parse::<u32>().map_err(|_| {
                warn!("Invalid shard_id parameter: {}", shard_str);
                StatusCode::BAD_REQUEST
            })?
        } else {
            warn!("shard_id parameter is missing");
            return Err(StatusCode::BAD_REQUEST);
        };

        database
            .execute(GuildSchema::insert(guild_id, shard_id))
            .await?;
        Ok(())
    } else {
        warn!("Non-bot user attempted to create guild entry");
        Err(StatusCode::FORBIDDEN)
    }
}

#[worker::send]
async fn delete_guild(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<(), StatusCode> {
    Ok(())
}
