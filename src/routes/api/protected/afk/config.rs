use axum::{extract::Path, Extension, Json};
use reqwest::StatusCode;
use sea_query::QueryStatement;
use serde::{Deserialize, Serialize};
use tracing::error;
use worker::console_debug;

use crate::{
    queries,
    schema::{AfkConfigSchema, UserSchema},
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

    let now: String = chrono::Utc::now().to_rfc3339();
    let per_guild = body.per_guild;
    let default_reason = body.default_reason.clone();
    let user_query = UserSchema::insert_if_not_exists(&user_id);
    let config_query = AfkConfigSchema::insert(&user_id, &per_guild, &default_reason, &now);
    let old_config_query = AfkConfigSchema::get(&user_id);

    let mult_queries: Vec<QueryStatement> = queries![user_query, old_config_query, config_query];
    let mut raw_results = database.batch_mixed(mult_queries).await.map_err(|e| {
        error!("Failed to set AFK config for user_id: {}\n{:?}", user_id, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to set AFK config"),
        )
    })?;

    // [0] = nothing, [1] = old config, [2] = new config
    raw_results.remove(0); // Remove the first element (nothing)
    let results = raw_results
        .iter()
        .map(|s| {
            serde_json::from_value(s.clone()).map_err(|e| {
                error!("Failed to deserialize AFK config result: {:?}", e);
                e
            })
        })
        .collect::<Result<Vec<Vec<AfkConfigSchema>>, _>>();

    match results {
        Ok(configs) => {
            let res = AfkConfigResponse {
                old_config: configs.first().and_then(|configs| configs.first().cloned()),
                new_config: (configs.last())
                    .and_then(|configs| configs.first().cloned())
                    .ok_or_else(|| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to set AFK config: New config not found".to_string(),
                        )
                    })?,
            };

            Ok(Json(res))
        }
        Err(e) => {
            error!(
                "Failed to deserialize AFK config results for user_id: {}\n{:?}",
                user_id, e
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set AFK config".to_string(),
            ));
        }
    }
}

#[worker::send]
pub async fn get(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<AfkConfigSchema>, (StatusCode, String)> {
    let config_query = AfkConfigSchema::get(&user_id);
    let results: Vec<AfkConfigSchema> = database.execute(config_query).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get AFK config: {:?}", e),
        )
    })?;

    if let Some(config) = results.first() {
        return Ok(Json(config.clone()));
    }
    Err((
        StatusCode::NOT_FOUND,
        "AFK config not found for user".to_string(),
    ))
}
