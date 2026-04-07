use axum::{Extension, Json, extract::Path};
use reqwest::StatusCode;
use tracing::{info, warn};

use crate::{
    schema::{ShardSchema, guild::GuildSchema},
    snowflake_protection,
    state::database::{Database, DatabaseExt},
};

/// Get specific shard information by ID
#[worker::send]
#[axum::debug_handler]
pub async fn get(
    Extension(database): Extension<Database>,
    Path(guild_id): Path<String>,
) -> Result<Json<ShardSchema>, (StatusCode, String)> {
    info!("Fetching shard information for guild: {}", guild_id);

    // Make sure the guild ID is a valid Discord snowflake
    snowflake_protection!(guild_id);

    let shards: Vec<ShardSchema> = (database.execute(GuildSchema::get_shard(&guild_id)).await)
        .map_err(|e| {
            warn!("Failed to get shard for guild {}: {:?}", guild_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get shard for guild".to_string(),
            )
        })?;

    if let Some(shard) = shards.first() {
        return Ok(Json(shard.clone()));
    }

    warn!("No shard found for guild: {}", guild_id);
    Err((
        StatusCode::NOT_FOUND,
        "No shard found for the specified guild".to_string(),
    ))
}
