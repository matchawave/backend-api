use axum::response::IntoResponse;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildData {
    id: String,
    #[serde_as(as = "BoolFromInt")]
    enabled: bool, // Indicates if the guild is enabled (1) or disabled (0)
}

impl GuildData {
    pub fn new(id: String, enabled: bool) -> Self {
        GuildData { id, enabled }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn to_owned(&self) -> Self {
        GuildData {
            id: self.id.clone(),
            enabled: self.enabled,
        }
    }
}

impl IntoResponse for GuildData {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string());
        axum::response::Response::builder()
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}

pub enum Guilds {
    Table,
    Id,
    Enabled,
}

impl Iden for Guilds {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Guilds::Table => "guilds",
                Guilds::Id => "id",
                Guilds::Enabled => "enabled",
            }
        )
        .unwrap();
    }
}
