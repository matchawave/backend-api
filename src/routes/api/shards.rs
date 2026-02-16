use axum::{
    extract::{Path, Request},
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_query::{Expr, Query, SelectStatement};
use tracing::{error, info, warn};
use worker::{Env, Stub};

use crate::{
    check_snowflake,
    durables::{
        bot::{BotDurable, ShardUpdatePayload},
        DurableFetch,
    },
    routes::api::shards,
    schema::{Guild, GuildSchema, ShardSchema},
    snowflake_protection,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_shards))
        .route("/{guild}", get(get_shard_by_guild))
        .route("/started/{count}", post(set_started_shards))
}

/// Get all stored shard information
#[worker::send]
// #[axum::debug_handler]
async fn get_all_shards(
    Extension(database): Extension<Database>,
    Extension(env): Extension<Env>,
    req: Request,
) -> Result<Json<Vec<ShardWithGuildCount>>, (StatusCode, String)> {
    info!("Fetching all shard information");
    let Ok(bot_token) = env.secret("DISCORD_BOT_TOKEN").map(|s| s.to_string()) else {
        error!("Failed to get bot token");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()));
    };

    let bot_durable = BotDurable::fetch_object(&env, &bot_token).map_err(|e| {
        error!("Failed to fetch bot durable object: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
    })?;

    let shard_datas = get_shards(&req, bot_durable).await.map_err(|e| {
        error!("Failed to get shards from durable object: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
    })?;

    let shard_infos: Vec<ShardSchema> =
        (database.execute(ShardSchema::get_all()).await).map_err(|e| {
            error!("Failed to get shard information from database: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
        })?;

    let queries = ShardedGuild::fetch(shard_datas.len() as u32);

    let guilds: Vec<Vec<ShardedGuild>> = database.batch(queries).await.map_err(|e| {
        error!("Failed to get guilds for shards: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
    })?;

    let output = (shard_datas.iter().zip(guilds).zip(shard_infos))
        .map(|((shard, guilds), info)| ShardWithGuildCount {
            shard: shard.shard_id,
            status: shard.status.clone(),
            latency: shard.latency_ms,
            guilds: guilds.len() as u32,
            users: shard.members,
            started: info.started_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(output))
}

async fn get_shards(req: &Request, bot_durable: Stub) -> Result<Vec<ShardUpdatePayload>, String> {
    let new_req = crate::copy_request(req, Some("/status"))
        .map_err(|e| format!("Failed to copy request: {}", e))?;

    let mut response = bot_durable
        .fetch_with_request(new_req)
        .await
        .map_err(|e| format!("Failed to fetch shard status from durable object: {}", e))?;

    let text_response = (response.text().await)
        .map_err(|e| format!("Failed to parse shard status response: {}", e))?;

    let shards: Vec<ShardUpdatePayload> = serde_json::from_str(&text_response)
        .map_err(|e| format!("Failed to deserialize shard status response: {}", e))?;

    Ok(shards)
}

/// Reset's the database's count of shards
/// with this, the api server knows how many shards the bot is running
#[worker::send]
async fn set_started_shards(
    Extension(database): Extension<Database>,
    Path(count): Path<u32>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    requested_user.bot_protection("Set Started Shards")?;
    info!("Setting started shards to {}", count);
    // ! NEVER DELETE PRE-EXISTING SHARDS, ONLY UPDATE THEM, OTHERWISE YOU LOSE GUILDS ASSOCIATED WITH THOSE SHARDS

    // Insert new shard entries
    let mut queries = Vec::with_capacity(count as usize);
    for shard_id in 0..count {
        let insert_stmt = ShardSchema::new_schema(shard_id);
        queries.push(insert_stmt);
    }

    if let Err(e) = database.batch(queries).await {
        warn!("Failed to insert new shards: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert new shards".to_string(),
        ));
    }
    info!("Inserted {} new shards", count);

    Ok(())
}
/// Get specific shard information by ID
#[worker::send]
#[axum::debug_handler]
async fn get_shard_by_guild(
    Extension(database): Extension<Database>,
    Path(guild_id): Path<String>,
) -> Result<Json<Vec<Shard>>, (StatusCode, String)> {
    info!("Fetching shard information for guild: {}", guild_id);

    // Make sure the guild ID is a valid Discord snowflake
    snowflake_protection!(guild_id);

    let shards: Vec<Shard> = (database
        .execute(GuildSchema::get_shard(guild_id.clone()))
        .await)
        .map_err(|e| {
            warn!("Failed to get shard for guild {}: {:?}", guild_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get shard for guild".to_string(),
            )
        })?;

    if shards.is_empty() {
        warn!("No shard found for guild: {}", guild_id);
        return Err((
            StatusCode::NOT_FOUND,
            "No shard found for the specified guild".to_string(),
        ));
    }
    Ok(Json(shards))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Shard {
    shard_id: u32,
}
#[derive(serde::Deserialize, Debug)]
struct ShardedGuild {
    #[serde(rename = "id")]
    guild_id: String,
    shard_id: u32,
}

impl ShardedGuild {
    pub fn fetch(shard_count: u32) -> Vec<SelectStatement> {
        (0..shard_count)
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
    shard: u32,
    status: String,
    latency: Option<u32>,
    guilds: u32,
    users: u32,
    started: Option<String>,
}
