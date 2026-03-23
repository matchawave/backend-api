use axum::{Extension, Json, extract::Path};
use reqwest::StatusCode;
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
    let config_query = AfkConfigSchema::get_and_insert(&user_id, &per_guild, &default_reason);

    let results: Vec<AfkConfigSchema> = database.execute(config_query).await.map_err(|e| {
        error!("Failed to set AFK config for user_id: {}\n{:?}", user_id, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set AFK config".into(),
        )
    })?;

    // list of 1 element: new_config only
    // list of 2 elements: old_config, new_config

    if results.is_empty() {
        error!(
            "Failed to set AFK config for user_id: {}. No results returned.",
            user_id
        );
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set AFK config: No results returned".to_string(),
        ));
    }

    if results.len() == 2
        && let Some(old) = results.first()
        && let Some(new) = results.last()
    {
        return Ok(Json(AfkConfigResponse {
            old_config: Some(old.clone()),
            new_config: new.clone(),
        }));
    }

    if results.len() == 1
        && let Some(new) = results.first()
    {
        return Ok(Json(AfkConfigResponse {
            old_config: None,
            new_config: new.clone(),
        }));
    }

    error!(
        "Unexpected number of results when setting AFK config for user_id: {}. Results: {:?}",
        user_id, results
    );
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to set AFK config: Unexpected number of results".to_string(),
    ))
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
