use std::fmt::Display;

use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettingsData {
    id: String,
    prefix: String,
    language: String,
    colour: Option<String>,
}

impl GuildSettingsData {
    pub fn new(id: String, prefix: &str, language: SupportedLanguages) -> Self {
        GuildSettingsData {
            id,
            prefix: prefix.to_string(),
            language: language.to_string(),
            colour: None,
        }
    }
    pub fn default(id: String) -> Self {
        GuildSettingsData {
            id,
            prefix: "!".to_string(),
            language: SupportedLanguages::English.to_string(),
            colour: None,
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum SupportedLanguages {
    #[serde(rename = "en")]
    #[default]
    English,
}

impl Display for SupportedLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SupportedLanguages::English => "en",
            }
        )
    }
}
