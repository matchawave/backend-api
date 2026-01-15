use axum::response::IntoResponse;
use sea_query::{DeleteStatement, Iden, InsertStatement, UpdateStatement};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

mod setting;
pub use setting::*;

use crate::state::database::Database;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildSchema {
    pub id: String,
    #[serde(deserialize_with = "deserialize_enabled")]
    pub enabled: bool, // Indicates if the guild is enabled (1) or disabled (0)
    pub shard_id: u32,
    pub last_updated: String,
    pub started_at: String,
}

fn deserialize_enabled<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: u8 = Deserialize::deserialize(deserializer)?;
    Ok(v != 0) // Convert 0 to false, any other value to true
}

impl IntoResponse for GuildSchema {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string());
        axum::response::Response::builder()
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}

#[derive(Iden)]
pub enum Guild {
    #[iden = "guilds"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "enabled"]
    Enabled,
    #[iden = "shard_id"]
    ShardId,
    #[iden = "added_at"]
    AddedAt,
    #[iden = "last_updated"]
    LastUpdated,
}

impl GuildSchema {
    pub fn insert<T: Into<String>>(guild_id: T, shard_id: u32) -> InsertStatement {
        sea_query::Query::insert()
            .into_table(Guild::Table)
            .columns(vec![Guild::Id, Guild::Enabled, Guild::ShardId])
            .values_panic(vec![
                guild_id.into().into(),
                1.into(), // Enabled by default
                shard_id.into(),
            ])
            .to_owned()
    }

    pub fn toggle<T: Into<String>>(guild_id: T, new_status: bool) -> UpdateStatement {
        let value = if new_status { 1 } else { 0 };
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::Enabled, value)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn set_shard<T: Into<String>>(guild_id: T, shard_id: u32) -> UpdateStatement {
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::ShardId, shard_id)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn update<T: Into<String>>(guild_id: T) -> UpdateStatement {
        let new_date = chrono::Utc::now().to_rfc3339();
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::LastUpdated, new_date)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn delete<T: Into<String>>(guild_id: T) -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }
}
