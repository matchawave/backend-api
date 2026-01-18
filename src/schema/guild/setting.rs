use reqwest::StatusCode;
use sea_query::{Expr, Iden, InsertStatement, OnConflict};
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettingsSchema {
    pub id: String,
    pub prefix: String,
    pub language: String,
    pub colour: Option<String>,
}

#[derive(Iden)]
pub enum GuildSettings {
    #[iden = "guild_settings"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "prefix"]
    Prefix,
    #[iden = "language"]
    Language,
    #[iden = "colour"]
    Colour,
}

#[derive(Debug, Serialize, Deserialize, Default, Display, Clone, Copy)]
pub enum SupportedLanguages {
    #[serde(rename = "en")]
    #[strum(to_string = "en")]
    #[default]
    English,
}

impl GuildSettingsSchema {
    pub fn default<T: Into<String>>(guild_id: T) -> Self {
        GuildSettingsSchema {
            id: guild_id.into(),
            prefix: "!".into(),
            language: SupportedLanguages::English.to_string(),
            colour: None,
        }
    }

    pub fn insert(schema: Self) -> Result<InsertStatement, StatusCode> {
        let language = schema.language;
        let prefix = schema.prefix;
        let colour = schema.colour;
        let columns = vec![
            GuildSettings::Id,
            GuildSettings::Prefix,
            GuildSettings::Language,
            GuildSettings::Colour,
        ];
        let values = vec![
            schema.id.into(),
            prefix.into(),
            language.into(),
            colour.into(),
        ];

        let on_conflict = OnConflict::columns(vec![
            GuildSettings::Prefix,
            GuildSettings::Language,
            GuildSettings::Colour,
        ]);

        match sea_query::Query::insert()
            .into_table(GuildSettings::Table)
            .columns(columns)
            .on_conflict(on_conflict)
            .values(values)
        {
            Ok(q) => Ok(q.to_owned()),
            Err(e) => {
                error!("Failed to build insert query: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn update_prefix<T: Into<String>>(
        guild_id: T,
        new_prefix: T,
    ) -> sea_query::UpdateStatement {
        sea_query::Query::update()
            .table(GuildSettings::Table)
            .value(GuildSettings::Prefix, new_prefix.into())
            .and_where(sea_query::Expr::col(GuildSettings::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn update_language<T: Into<String>>(
        guild_id: T,
        new_language: SupportedLanguages,
    ) -> sea_query::UpdateStatement {
        sea_query::Query::update()
            .table(GuildSettings::Table)
            .value(GuildSettings::Language, new_language.to_string())
            .and_where(sea_query::Expr::col(GuildSettings::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn delete<T: Into<String>>(guild_id: T) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(GuildSettings::Table)
            .and_where(sea_query::Expr::col(GuildSettings::Id).eq(guild_id.into()))
            .returning_all()
            .to_owned()
    }

    pub fn get_by_id<T: Into<String>>(guild_id: T) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(GuildSettings::Table)
            .and_where(Expr::col(GuildSettings::Id).eq(guild_id.into()))
            .to_owned()
    }
}
