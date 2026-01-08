use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Json, Router};
use reqwest::StatusCode;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use tracing::{error, info};

use crate::{
    schema::{ShardData, Shards},
    state::database::Databases,
};

pub fn router() -> Router {
    Router::new().route("/", get(get_all_shards))
    // .route("/{shard_id}", get(get_shard_by_id))
}

/// Get all stored shard information
#[worker::send]
async fn get_all_shards(
    Extension(databases): Extension<Databases>,
) -> Result<Json<Vec<ShardData>>, (StatusCode, String)> {
    info!("Fetching all shard information");

    let query = Query::select()
        .from(Shards::Table)
        .columns(vec![
            Shards::ShardId,
            Shards::Status,
            Shards::LatencyMs,
            Shards::Servers,
            Shards::Members,
        ])
        .build(SqliteQueryBuilder);

    let shards = (databases.general.select::<ShardData>(query))
        .await
        .map_err(|e| (e, format!("Failed to get shards: {}", e)))?;

    Ok(Json(shards))
}

/// Get specific shard information by ID
#[worker::send]
async fn get_shard_by_id(
    Extension(databases): Extension<Databases>,
    Path(shard_id): Path<u32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Fetching shard information for shard_id: {}", shard_id);

    let query = Query::select()
        .from(Shards::Table)
        .columns(vec![
            Shards::ShardId,
            Shards::Status,
            Shards::LatencyMs,
            Shards::Servers,
            Shards::Members,
            Shards::LastUpdated,
        ])
        .and_where(Expr::col(Shards::ShardId).eq(shard_id))
        .build(SqliteQueryBuilder);

    let shards = databases
        .general
        .select::<ShardData>(query)
        .await
        .map_err(|e| (e, format!("Failed to get shard: {}", e)))?;

    if shards.is_empty() {
        info!("Shard {} not found", shard_id);
        return Err((
            StatusCode::NOT_FOUND,
            format!("Shard with ID {} not found", shard_id),
        ));
    }

    if let Some(shard) = shards.into_iter().next() {
        Ok(Json(shard))
    } else {
        error!("Multiple shards found for ID {}", shard_id);
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Multiple shards found for ID {}", shard_id),
        ))
    }
}
