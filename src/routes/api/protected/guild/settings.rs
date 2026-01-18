use std::fmt::Display;

use axum::{
    extract::{Multipart, Path},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_query::{Expr, OnConflict, Query, SqliteQueryBuilder};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};
use worker::{console_debug, console_log};

use crate::{
    schema::{GuildSettings, GuildSettingsSchema, SupportedLanguages},
    state::database::{Database, DatabaseExt},
};

pub fn router() -> Router {
    Router::new().route(
        "/",
        get(get_setting).post(update_setting).delete(delete_setting),
    )
}

#[derive(Deserialize, Debug)]
struct ConfigBody {
    prefix: Option<String>,
    language: Option<SupportedLanguages>,
    colour: Option<String>,
}

#[worker::send]
async fn get_setting(
    Path(id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<GuildSettingsSchema>, StatusCode> {
    console_log!("Getting settings for guild {}", id);
    let query = GuildSettingsSchema::get_by_id(id.clone());

    let settings: Vec<GuildSettingsSchema> = database.execute(query).await.map_err(|e| {
        error!("Failed to get guild settings for ID {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    if settings.is_empty() {
        warn!("Guild settings for ID {} not found", id);
        return Ok(Json(GuildSettingsSchema::default(id)));
    }
    if settings.len() > 1 {
        error!("Multiple guild settings found for ID {}", id);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(settings[0].to_owned()))
}

#[worker::send]
async fn update_setting(
    // Update guild settings, if the guild does not exist, it will be created
    Path(id): Path<String>,
    Extension(database): Extension<Database>,
    Json(body): Json<ConfigBody>,
) -> Result<Json<GuildSettingsSchema>, (StatusCode, String)> {
    console_debug!("{:?}", body);
    if let Some(prefix) = &body.prefix {
        if prefix.len() > 10 {
            error!("Prefix too long for guild {}", id);
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Prefix too long for guild {}", id),
            ));
        } else if prefix.is_empty() {
            error!("Prefix cannot be empty for guild {}", id);
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Prefix cannot be empty for guild {}", id),
            ));
        }
    }

    let settings = GuildSettingsSchema {
        id: id.clone(),
        prefix: body.prefix.clone().unwrap_or("!".into()),
        language: (body.language.unwrap_or(SupportedLanguages::English)).to_string(),
        colour: body.colour.clone(),
    };

    let query = GuildSettingsSchema::insert(settings.clone()).map_err(|e| {
        error!("Failed to build insert query for guild {}: {}", id, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    if let Err(e) = database.execute(query).await {
        error!("Failed to update guild settings for ID {}: {}", id, e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ));
    }
    Ok(Json(settings))
}

#[worker::send]
async fn delete_setting(
    Path(id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<GuildSettingsSchema>, StatusCode> {
    let query = GuildSettingsSchema::delete(id.clone());

    let deleted_settings: Vec<GuildSettingsSchema> =
        database.execute(query).await.map_err(|e| {
            error!("Failed to delete guild settings for ID {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(GuildSettingsSchema::default(id)))
}
