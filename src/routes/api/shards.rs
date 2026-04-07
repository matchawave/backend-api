use axum::{
    Extension, Json, Router,
    extract::{Path, Request},
    routing::{get, post},
};
use reqwest::StatusCode;
use sea_query::{Expr, SelectStatement};
use tracing::{error, info, warn};
use worker::{Env, Stub};

use crate::{
    durables::{
        DurableFetch,
        bot::{BotDurable, ShardUpdatePayload},
    },
    schema::{ShardSchema, Shards, guild::Guild},
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_shards))
        .route("/{shard_id}", get(get_shard).post(set_shard_started))
        .route("/started/{count}", post(set_started_shards))
}

/// Get all stored shard information
#[worker::send]
#[axum::debug_handler]
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

    let shards_with_guild_counts: Vec<ShardGuildCount> = database
        .execute(get_guild_counts_and_shards())
        .await
        .map_err(|e| {
            error!("Failed to get guild counts from database: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
        })?;

    let output =
        (shards_with_guild_counts.iter())
            .map(|info| {
                let shard = shard_datas.get(info.shard_id as usize).cloned().unwrap_or(
                    ShardUpdatePayload {
                        shard_id: info.shard_id,
                        ..Default::default()
                    },
                );
                ShardWithGuildCount {
                    shard: shard.shard_id,
                    status: shard.status.clone(),
                    latency: shard.latency_ms,
                    guilds: info.guild_count,
                    users: shard.members,
                    started: info.started_at.clone(),
                }
            })
            .collect::<Vec<_>>();

    Ok(Json(output))
}

#[derive(serde::Deserialize)]
pub struct ShardGuildCount {
    #[serde(rename = "id")]
    shard_id: u32,
    started_at: Option<String>,
    guild_count: u32,
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

fn get_guild_counts_and_shards() -> SelectStatement {
    let shard_id_col = (Shards::Table, Shards::Id);
    let shard_started_col = (Shards::Table, Shards::StartedAt);
    let guild_id_col = (Guild::Table, Guild::Id);
    let guild_shard_col = (Guild::Table, Guild::ShardId);

    SelectStatement::new()
        .columns([shard_id_col, shard_started_col])
        .from(Shards::Table)
        .expr_as(Expr::col(guild_id_col).count(), "guild_count")
        .left_join(
            Guild::Table,
            Expr::col(shard_id_col).equals(guild_shard_col),
        )
        .group_by_col(shard_id_col)
        .order_by(shard_id_col, sea_query::Order::Asc)
        .to_owned()
}

#[worker::send]
#[axum::debug_handler]
async fn get_shard(
    Extension(database): Extension<Database>,
    Path(shard_id): Path<String>,
) -> Result<Json<Vec<ShardSchema>>, (StatusCode, String)> {
    info!("Fetching shard information for shard: {}", shard_id);

    let shard_id: u32 = shard_id.parse().map_err(|e| {
        warn!("Invalid shard ID provided: {}: {:?}", shard_id, e);
        (
            StatusCode::BAD_REQUEST,
            "Invalid shard ID provided".to_string(),
        )
    })?;

    let shards: Vec<ShardSchema> = (database.execute(ShardSchema::get_by_id(shard_id)).await)
        .map_err(|e| {
            warn!("Failed to get shard for shard {}: {:?}", shard_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get shard for shard".to_string(),
            )
        })?;

    if shards.is_empty() {
        warn!("No shard found for shard: {}", shard_id);
        return Err((
            StatusCode::NOT_FOUND,
            "No shard found for the specified shard".to_string(),
        ));
    }
    Ok(Json(shards))
}

#[worker::send]
#[axum::debug_handler]
async fn set_shard_started(
    Extension(database): Extension<Database>,
    Path(shard_id): Path<String>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    requested_user.bot_protection("Shard Starting")?;
    info!("Setting shard {} as started", shard_id);

    let shard_id: u32 = shard_id.parse().map_err(|e| {
        warn!("Invalid shard ID provided: {}: {:?}", shard_id, e);
        (
            StatusCode::BAD_REQUEST,
            "Invalid shard ID provided".to_string(),
        )
    })?;

    (database
        .execute(ShardSchema::set_started_at(shard_id))
        .await)
        .map_err(|e| {
            error!("Failed to set shard {} as started: {:?}", shard_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set shard as started".to_string(),
            )
        })?;
    info!("Shard {} set as started", shard_id);
    Ok(())
}

/// Reset's the database's count of shards
/// with this, the api server knows how many shards the bot is running
#[worker::send]
#[axum::debug_handler]
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

    let _: Vec<()> = database.batch(&queries).await.map_err(|e| {
        error!("Failed to insert new shards: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert new shards".to_string(),
        )
    })?;
    info!("Inserted {} new shards", count);

    Ok(())
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
