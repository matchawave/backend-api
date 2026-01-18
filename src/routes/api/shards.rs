use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_query::{Expr, Query, SelectStatement};
use tracing::{error, info};

use crate::{
    schema::{Guild, ShardSchema},
    state::database::{Database, DatabaseExt},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_shards))
        // .route("/{shard_id}", get(get_shard_by_id))
        .route("/started/{count}", post(set_started_shards))
}

/// Get all stored shard information
#[worker::send]
async fn get_all_shards(
    Extension(database): Extension<Database>,
) -> Result<Json<Vec<ShardWithGuildCount>>, (StatusCode, String)> {
    info!("Fetching all shard information");

    let shards: Vec<ShardSchema> = (database.execute(ShardSchema::get_all()).await)
        .map_err(|e| (e, format!("Failed to get shards: {}", e)))?;

    let queries = ShardedGuild::fetch(shards.len() as u32);
    let guilds: Vec<Vec<ShardedGuild>> = database
        .batch(queries)
        .await
        .map_err(|e| (e, format!("Failed to get guild counts: {}", e)))?;

    let counts = guilds
        .into_iter()
        .map(|g| g.len() as u32)
        .collect::<Vec<u32>>();
    let output = shards
        .into_iter()
        .zip(counts)
        .map(|(shard, count)| ShardWithGuildCount {
            shard,
            guild_count: count,
        })
        .collect::<Vec<ShardWithGuildCount>>();

    Ok(Json(output))
}

/// Reset's the database's count of shards
/// with this, the api server knows how many shards the bot is running
#[worker::send]
async fn set_started_shards(
    Extension(database): Extension<Database>,
    Path(count): Path<u32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Setting started shards to {}", count);

    // Clear existing shards
    let shards: Vec<ShardSchema> = database
        .execute(ShardSchema::delete_all())
        .await
        .map_err(|e| (e, format!("Failed to clear shards: {}", e)))?;

    info!("Cleared existing shards: {}", shards.len());

    // Insert new shard entries
    let mut queries = Vec::with_capacity(count as usize);
    for shard_id in 0..count {
        let insert_stmt = ShardSchema::new_schema(shard_id, "offline".to_string());

        queries.push(insert_stmt);
    }

    if let Err(e) = database.batch(queries).await {
        return Err((e, format!("Failed to insert new shards: {}", e)));
    }
    info!("Inserted {} new shards", count);

    Ok(StatusCode::OK)
}
/// Get specific shard information by ID
#[worker::send]
async fn get_shard_by_guild(
    Extension(database): Extension<Database>,
    Path(guild_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Fetching shard information for guild: {}", guild_id);

    let shards: Vec<ShardSchema> = database
        .execute(ShardSchema::get_by_guild(guild_id.clone()))
        .await
        .map_err(|e| (e, format!("Failed to get shard: {}", e)))?;

    if shards.is_empty() {
        info!("guild {} not found", guild_id);
        return Err((
            StatusCode::NOT_FOUND,
            format!("Guild with ID {} not found", guild_id),
        ));
    }

    if let Some(shard) = shards.into_iter().next() {
        Ok(Json(shard))
    } else {
        error!("Multiple shards found for ID {}", guild_id);
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Multiple shards found for ID {}", guild_id),
        ))
    }
}

#[derive(serde::Deserialize)]
struct ShardedGuild {
    guild_id: String,
    shard_id: u32,
}

impl ShardedGuild {
    pub fn fetch(shard_count: u32) -> Vec<SelectStatement> {
        (1..=shard_count)
            .map(|shard_id| {
                Query::select()
                    .from(Guild::Table)
                    .and_where(Expr::col(Guild::ShardId).eq(shard_id))
                    .columns(vec![Guild::Id, Guild::ShardId])
                    .to_owned()
            })
            .collect()
    }
}

#[derive(serde::Serialize)]
struct ShardWithGuildCount {
    shard: ShardSchema,
    guild_count: u32,
}
