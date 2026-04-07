use axum::{Extension, Json, Router, extract::Path, routing::get};
use reqwest::StatusCode;
use sea_query::{DeleteStatement, Expr, InsertStatement, SelectStatement};
use serde::{Deserialize, Serialize};

use crate::{
    schema::guild::{
        ColourSchema, Colours, LanguageSchema, Languages, PrefixSchema, Prefixes, TimezoneSchema,
        Timezones,
    },
    services::streaming::StreamableSchema,
    state::database::{Database, DatabaseExt},
};

mod colour;
mod language;
mod prefix;
mod timezone;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_settings).post(set_settings).delete(delete_settings),
        )
        .route(
            "/prefix",
            get(prefix::get).post(prefix::set).delete(prefix::delete),
        )
        .route(
            "/language",
            get(language::get)
                .post(language::set)
                .delete(language::delete),
        )
        .route(
            "/timezone",
            get(timezone::get)
                .post(timezone::set)
                .delete(timezone::delete),
        )
        .route(
            "/colour",
            get(colour::get).post(colour::set).delete(colour::delete),
        )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsBody {
    pub prefix: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub colour: Option<String>,
}

impl SettingsBody {
    pub fn get(guild_id: &str) -> SelectStatement {
        let prefix_col = (Prefixes::Table, Prefixes::GuildId);
        let language_col = (Languages::Table, Languages::GuildId);
        let timezone_col = (Timezones::Table, Timezones::GuildId);
        let colour_col = (Colours::Table, Colours::GuildId);
        SelectStatement::new()
            .column(Prefixes::Prefix)
            .from(Prefixes::Table)
            .and_where(Expr::col(prefix_col).eq(guild_id))
            .left_join(Languages::Table, Expr::col(prefix_col).equals(language_col))
            .left_join(Timezones::Table, Expr::col(prefix_col).equals(timezone_col))
            .left_join(Colours::Table, Expr::col(prefix_col).equals(colour_col))
            .to_owned()
    }

    pub fn set(guild_id: &str, data: &SettingsBody) -> Vec<InsertStatement> {
        let mut queries = Vec::new();
        if let Some(prefix) = &data.prefix {
            queries.push(PrefixSchema::insert(guild_id, prefix));
        }
        if let Some(language) = &data.language {
            queries.push(LanguageSchema::insert(guild_id, language));
        }
        if let Some(timezone) = &data.timezone {
            queries.push(TimezoneSchema::insert(guild_id, timezone));
        }
        if let Some(colour) = &data.colour {
            queries.push(ColourSchema::insert(guild_id, colour));
        }
        queries
    }

    pub fn delete(guild_id: &str, data: &SettingsBody) -> Vec<DeleteStatement> {
        let mut queries = Vec::new();
        if data.prefix.is_none() {
            queries.push(PrefixSchema::delete(guild_id));
        }
        if data.language.is_none() {
            queries.push(LanguageSchema::delete(guild_id));
        }
        if data.timezone.is_none() {
            queries.push(TimezoneSchema::delete(guild_id));
        }
        if data.colour.is_none() {
            queries.push(ColourSchema::delete(guild_id));
        }
        queries
    }
}

impl StreamableSchema for SettingsBody {
    fn all_by_batch(batch_size: u64, offset: u64) -> sea_query::SelectStatement {
        let prefix_col = (Prefixes::Table, Prefixes::GuildId);
        let language_col = (Languages::Table, Languages::GuildId);
        let timezone_col = (Timezones::Table, Timezones::GuildId);
        let colour_col = (Colours::Table, Colours::GuildId);
        SelectStatement::new()
            .column(Prefixes::Prefix)
            .from(Prefixes::Table)
            .left_join(Languages::Table, Expr::col(prefix_col).equals(language_col))
            .left_join(Timezones::Table, Expr::col(prefix_col).equals(timezone_col))
            .left_join(Colours::Table, Expr::col(prefix_col).equals(colour_col))
            .limit(batch_size)
            .offset(offset)
            .to_owned()
    }
}

#[worker::send]
#[axum::debug_handler]
pub async fn get_settings(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<SettingsBody>, (StatusCode, String)> {
    let query = SettingsBody::get(&guild_id);
    let settings: Vec<SettingsBody> = (database.execute(query).await).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get settings: {:?}", e),
        )
    })?;
    if let Some(settings) = settings.first() {
        Ok(Json(settings.clone()))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "{} not found for this guild".to_string(),
        ))
    }
}

#[worker::send]
#[axum::debug_handler]
pub async fn set_settings(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Json(new_data): Json<SettingsBody>,
) -> Result<(), (StatusCode, String)> {
    let queries = SettingsBody::set(&guild_id, &new_data);
    let _: () = database.batch(&queries).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to set settings: {:?}", e),
        )
    })?;
    Ok(())
}

#[worker::send]
#[axum::debug_handler]
pub async fn delete_settings(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Json(data): Json<SettingsBody>,
) -> Result<(), (StatusCode, String)> {
    let queries = SettingsBody::delete(&guild_id, &data);
    let _: () = database.batch(&queries).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete settings: {:?}", e),
        )
    })?;
    Ok(())
}

#[macro_export]
macro_rules! create_settings_path {
    ($struct:ident) => {
        use axum::{Extension, Json, extract::Path};
        use reqwest::StatusCode;
        use $crate::state::database::{Database, DatabaseExt};

        #[worker::send]
        #[axum::debug_handler]
        pub async fn get(
            Path(guild_id): Path<String>,
            Extension(database): Extension<Database>,
        ) -> Result<Json<$struct>, (StatusCode, String)> {
            let query = $struct::get(&guild_id);
            let prefix: Vec<$struct> = (database.execute(query).await).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get {}: {:?}", stringify!($struct), e),
                )
            })?;

            if let Some(prefix) = prefix.first() {
                Ok(Json(prefix.clone()))
            } else {
                Err((
                    StatusCode::NOT_FOUND,
                    format!("{} not found for this guild", stringify!($struct)),
                ))
            }
        }

        #[worker::send]
        #[axum::debug_handler]
        pub async fn set(
            Path(guild_id): Path<String>,
            Extension(database): Extension<Database>,
            Json(new_data): Json<String>,
        ) -> Result<(), (StatusCode, String)> {
            let query = $struct::insert(&guild_id, &new_data);
            let _: () = (database.execute(query).await).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to set {}: {:?}", stringify!($struct), e),
                )
            })?;

            Ok(())
        }

        #[worker::send]
        #[axum::debug_handler]
        pub async fn delete(
            Path(guild_id): Path<String>,
            Extension(database): Extension<Database>,
        ) -> Result<(), (StatusCode, String)> {
            let query = $struct::delete(&guild_id);
            let _: () = (database.execute(query).await).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to delete {}: {:?}", stringify!($struct), e),
                )
            })?;

            Ok(())
        }
    };
}
