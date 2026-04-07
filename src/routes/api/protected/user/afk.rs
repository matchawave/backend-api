use crate::{
    schema::{AfkStatusSchema, user::AfkConfigSchema},
    snowflake_protection,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use reqwest::StatusCode;
use sea_query::QueryStatement;
use serde::Deserialize;
use tracing::{debug, error, warn};

#[derive(Debug, Deserialize)]
pub struct GuildQuery {
    guild_id: Option<String>,
}

#[worker::send]
#[axum::debug_handler]
pub async fn set(
    Path(user_id): Path<String>,
    Query(params): Query<GuildQuery>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(reason): Json<String>,
) -> Result<Json<AfkStatusSchema>, (StatusCode, String)> {
    debug!(
        "Setting AFK status for user_id: {}, guild_id: {:?}, reason: {}",
        user_id, params.guild_id, reason
    );
    requested_user.bot_protection("Set AFK Status")?;
    let guild_id = match params.guild_id {
        Some(guild_id) => {
            snowflake_protection!(guild_id);
            Some(guild_id)
        }
        None => None,
    };
    let afk_query = AfkStatusSchema::insert(&user_id, &guild_id, &reason);

    let result: Vec<AfkStatusSchema> = (database.execute(afk_query).await).map_err(|e| {
        warn!("Failed to set AFK status: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set AFK status".to_string(),
        )
    })?;

    let status = result.first().unwrap();

    Ok(Json(status.clone()))
}

#[worker::send]
#[axum::debug_handler]
pub async fn get(
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
#[axum::debug_handler]
pub async fn remove(
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

#[derive(Debug, serde::Deserialize)]
pub struct NewAfkConfigBody {
    per_guild: Option<bool>,
    default_reason: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AfkConfigResponse {
    old_config: Option<AfkConfigSchema>,
    new_config: AfkConfigSchema,
}

#[worker::send]
#[axum::debug_handler]
pub async fn set_config(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(body): Json<NewAfkConfigBody>,
) -> Result<Json<AfkConfigResponse>, (StatusCode, String)> {
    requested_user.bot_protection("Set AFK Config")?;

    let per_guild = body.per_guild;
    let default_reason = body.default_reason.clone();
    let get_config = QueryStatement::Select(AfkConfigSchema::get(&user_id));
    let insert_config = QueryStatement::Insert(AfkConfigSchema::insert(
        &user_id,
        &per_guild,
        &default_reason,
    ));

    let results = database
        .simple_batch_mixed::<AfkConfigSchema, AfkConfigSchema, (), ()>(&[
            get_config,
            insert_config,
        ])
        .await
        .map_err(|e| {
            error!("Failed to set AFK config for user_id: {}\n{:?}", user_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set AFK config".into(),
            )
        })?;

    // list of 1 element: new_config only
    // list of 2 elements: old_config, new_config

    if let Some(new) = results.insert.and_then(|v| v.first().cloned()) {
        let old = results.select.and_then(|v| v.first().cloned());
        return Ok(Json(AfkConfigResponse {
            old_config: old.clone(),
            new_config: new.clone(),
        }));
    }
    error!(
        "Failed to set AFK config for user_id: {}. No results returned.",
        user_id
    );
    return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to set AFK config: No results returned".to_string(),
    ));
}

#[worker::send]
#[axum::debug_handler]
pub async fn get_config(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<Option<AfkConfigSchema>>, (StatusCode, String)> {
    let config_query = AfkConfigSchema::get(&user_id);
    let results: Vec<AfkConfigSchema> = database.execute(config_query).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get AFK config: {:?}", e),
        )
    })?;

    Ok(Json(results.first().cloned()))
}
