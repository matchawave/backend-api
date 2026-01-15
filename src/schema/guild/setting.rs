use std::fmt::Display;

use sea_query::{Iden, InsertStatement};
use serde::{Deserialize, Serialize};
use strum::Display;

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

#[derive(Serialize, Deserialize, Default, Display, Clone, Copy)]
pub enum SupportedLanguages {
    #[serde(rename = "en")]
    #[strum(to_string = "en")]
    #[default]
    English,
}

impl GuildSettingsSchema {
    pub fn insert<T: Into<String>>(guild_id: T) -> InsertStatement {
        let default_language = SupportedLanguages::English.to_string();
        let default_prefix = "!";
        let default_colour: Option<String> = None;
        let columns = vec![
            GuildSettings::Id,
            GuildSettings::Prefix,
            GuildSettings::Language,
            GuildSettings::Colour,
        ];
        let values = vec![
            guild_id.into().into(),
            default_prefix.into(),
            default_language.into(),
            default_colour.into(),
        ];
        sea_query::Query::insert()
            .into_table(GuildSettings::Table)
            .columns(columns)
            .values_panic(values)
            .to_owned()
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
}
