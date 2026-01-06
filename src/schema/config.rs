use std::fmt::Display;

use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettingsData {
    id: String,
    prefix: String,
    language: String,
}

impl GuildSettingsData {
    pub fn new(id: String, prefix: &str, language: SupportedLanguages) -> Self {
        GuildSettingsData {
            id,
            prefix: prefix.to_string(),
            language: language.to_string(),
        }
    }
    pub fn default(id: String) -> Self {
        GuildSettingsData {
            id,
            prefix: "!".to_string(),
            language: SupportedLanguages::English.to_string(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn prefix(&self) -> &String {
        &self.prefix
    }
    pub fn language(&self) -> &String {
        &self.language
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    pub fn to_owned(&self) -> Self {
        GuildSettingsData {
            id: self.id.clone(),
            prefix: self.prefix.clone(),
            language: self.language.clone(),
        }
    }
}

pub enum GuildSettings {
    Table,
    Id,
    Prefix,
    Language,
}

impl Iden for GuildSettings {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                GuildSettings::Table => "guild_settings",
                GuildSettings::Id => "id",
                GuildSettings::Prefix => "prefix",
                GuildSettings::Language => "language",
            }
        )
        .unwrap();
    }
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
