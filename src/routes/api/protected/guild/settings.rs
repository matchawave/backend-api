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
    state::database::Database,
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
}

#[worker::send]
async fn get_setting(
    Path(id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<GuildSettingsSchema>, StatusCode> {
    console_log!("Getting settings for guild {}", id);
    let query = Query::select()
        .column(GuildSettings::Id)
        .column(GuildSettings::Prefix)
        .column(GuildSettings::Language)
        .from(GuildSettings::Table)
        .and_where(Expr::col(GuildSettings::Id).eq(id.clone()))
        .build(SqliteQueryBuilder);

    let settings = database.select::<GuildSettingsSchema>(query).await?;
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

    let new_prefix = body.prefix.clone().unwrap_or_else(|| "!".to_string());
    let new_language = body.language.clone().unwrap_or(SupportedLanguages::English);

    let query = Query::insert()
        .into_table(GuildSettings::Table)
        .columns(vec![
            GuildSettings::Id,
            GuildSettings::Prefix,
            GuildSettings::Language,
        ])
        .on_conflict(
            OnConflict::column(GuildSettings::Id)
                .update_column(GuildSettings::Prefix)
                .update_column(GuildSettings::Language)
                .to_owned(),
        )
        .values(vec![
            Expr::value(id.clone()),
            Expr::value(new_prefix),
            Expr::value(new_language.to_string()),
        ])
        .unwrap()
        .returning_all()
        .build(SqliteQueryBuilder);

    let updated_settings = database
        .select::<GuildSettingsSchema>(query)
        .await
        .map_err(|err| {
            error!(
                "Database error while updating settings for guild {}: {}",
                id, err
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "".into())
        })?;
    Ok(Json(updated_settings[0].to_owned()))
}

#[worker::send]
async fn delete_setting(
    Path(id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<GuildSettingsSchema>, StatusCode> {
    let query = Query::delete()
        .from_table(GuildSettings::Table)
        .and_where(Expr::col(GuildSettings::Id).eq(id.clone()))
        .build(SqliteQueryBuilder);

    database.insert(query).await?;
    Ok(Json(GuildSettingsSchema::default(id)))
}
