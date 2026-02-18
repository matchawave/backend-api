use axum::{
    body::{Body, Bytes},
    extract::{Path, Query},
    http::Response,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use reqwest::StatusCode;

use futures::stream::{self, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use worker::console_log;

use crate::{
    schema::{AfkStatusSchema, UserSchema},
    snowflake_protection,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/user/{user_id}",
            get(get_afk).post(set_afk).delete(remove_afk),
        )
        .route("/", get(get_all_afk_streaming))
        .route("/guild/{guild_id}", get(get_guild_afk))
}

#[derive(Debug, Clone, Deserialize)]
struct NewAfkBody {
    guild_id: Option<String>,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct GuildQuery {
    guild_id: Option<String>,
}

#[axum::debug_handler]
#[worker::send]
async fn set_afk(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(body): Json<NewAfkBody>,
) -> Result<Json<AfkStatusSchema>, (StatusCode, String)> {
    debug!(
        "Setting AFK status for user_id: {}, guild_id: {:?}, reason: {}",
        user_id, body.guild_id, body.reason
    );
    requested_user.bot_protection("Set AFK Status")?;
    let guild_id = match body.guild_id {
        Some(guild_id) => {
            snowflake_protection!(guild_id);
            Some(guild_id)
        }
        None => None,
    };
    let current_time = chrono::Utc::now().to_rfc3339();
    let user_query = UserSchema::insert_if_not_exists(&user_id);
    let afk_query = AfkStatusSchema::insert(&user_id, &guild_id, &body.reason, &current_time);

    database
        .batch(vec![user_query, afk_query])
        .await
        .map_err(|e| {
            warn!("Failed to set AFK status: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set AFK status".to_string(),
            )
        })?;

    Ok(Json(AfkStatusSchema {
        user_id,
        guild_id,
        reason: body.reason,
        created_at: current_time,
    }))
}

#[worker::send]
async fn get_afk(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    // Extension(requested_user): Extension<RequestedUser>,
) -> Result<Json<Vec<AfkStatusSchema>>, (StatusCode, String)> {
    let user_query = AfkStatusSchema::get(&user_id);
    let users: Vec<AfkStatusSchema> = database.execute(user_query).await.map_err(|e| {
        warn!("Failed to get AFK status: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get AFK status".to_string(),
        )
    })?;
    Ok(Json(users))
}

#[worker::send]
async fn remove_afk(
    Path(user_id): Path<String>,
    Query(params): Query<GuildQuery>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<Json<AfkStatusSchema>, (StatusCode, String)> {
    requested_user.bot_protection("Remove AFK Status")?;
    let guild_id = match params.guild_id {
        Some(guild_id) => {
            snowflake_protection!(guild_id);
            Some(guild_id)
        }
        None => None,
    };
    let afk_query = AfkStatusSchema::delete(&user_id, &guild_id);
    let users: Vec<AfkStatusSchema> = database.execute(afk_query).await.map_err(|e| {
        warn!("Failed to remove AFK status: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to remove AFK status".to_string(),
        )
    })?;
    if users.is_empty() {
        return Err((StatusCode::NOT_FOUND, "User is not AFK".to_string()));
    }

    if let Some(first) = users.first() {
        if first.user_id != user_id {
            return Err((StatusCode::NOT_FOUND, "User is not AFK".to_string()));
        }

        return Ok(Json(first.clone()));
    }
    Err((StatusCode::NOT_FOUND, "User is not AFK".to_string()))
}

#[worker::send]
async fn get_all_afk_streaming(
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    requested_user.bot_protection("Get All AFK Statuses")?;
    debug!("Fetching all AFK statuses from the database");
    let afk_query = AfkStatusSchema::all();
    let users: Vec<AfkStatusSchema> = database.execute(afk_query).await.map_err(|e| {
        warn!("Failed to get AFK statuses: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get AFK statuses".to_string(),
        )
    })?;

    let stream_output = stream::iter(users).map(|user| {
        let json = serde_json::to_string(&user).unwrap_or_else(|_| "{}".to_string());
        console_log!("Streaming AFK status: {}", json);
        Ok::<_, std::io::Error>(Bytes::from(format!("{}\n", json)))
    });

    let body = Body::from_stream(stream_output);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-ndjson")
        .body(body)
        .map_err(|e| {
            warn!("Failed to build response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build response".to_string(),
            )
        })
}

#[worker::send]
async fn get_guild_afk(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<Json<Vec<AfkStatusSchema>>, (StatusCode, String)> {
    requested_user.bot_protection("Get Guild AFK Statuses")?;
    snowflake_protection!(guild_id);
    let afk_query = AfkStatusSchema::get_guild(&guild_id);
    let users: Vec<AfkStatusSchema> = database.execute(afk_query).await.map_err(|e| {
        warn!("Failed to get guild AFK statuses: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get guild AFK statuses".to_string(),
        )
    })?;

    Ok(Json(users))
}
