use axum::{Extension, Json, extract::Path};
use reqwest::StatusCode;
use sea_query::QueryStatement;
use serde::{Deserialize, Serialize};
use tracing::error;
use worker::console_debug;

use crate::{
    schema::AfkConfigSchema,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

#[derive(Debug, Deserialize)]
pub struct NewAfkConfigBody {
    per_guild: Option<bool>,
    default_reason: Option<String>,
}

#[derive(Serialize)]
pub struct AfkConfigResponse {
    old_config: Option<AfkConfigSchema>,
    new_config: AfkConfigSchema,
}

#[worker::send]
#[axum::debug_handler]
pub async fn set(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(body): Json<NewAfkConfigBody>,
) -> Result<Json<AfkConfigResponse>, (StatusCode, String)> {
    requested_user.bot_protection("Set AFK Config")?;
    console_debug!(
        "Setting AFK config for user_id: {}, per_guild: {:?}, default_reason: {:?}",
        user_id,
        body.per_guild,
        body.default_reason
    );

    let per_guild = body.per_guild;
    let default_reason = body.default_reason.clone();
    let get_config = QueryStatement::Select(AfkConfigSchema::get(&user_id));
    let insert_config = QueryStatement::Insert(AfkConfigSchema::insert(
        &user_id,
        &per_guild,
        &default_reason,
    ));

    let results = database
        .batch_mixed::<AfkConfigSchema, AfkConfigSchema, (), ()>(&[get_config, insert_config])
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

    if let old = results.select.and_then(|v| v.first().cloned())
        && let Some(new) = results.insert.and_then(|v| v.first().cloned())
    {
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
pub async fn get(
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
