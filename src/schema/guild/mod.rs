use super::deserialize_bool;
use sea_query::{DeleteStatement, Iden, InsertStatement, UpdateStatement};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

mod configuration;
mod misc;
mod permission;
mod setting;
mod utility;

pub use configuration::*;

pub use misc::*;
pub use permission::*;
pub use setting::*;
pub use utility::*;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildSchema {
    pub id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool, // Indicates if the guild is enabled (1) or disabled (0)
    pub shard_id: u32,
    pub last_updated: String,
    pub started_at: String,
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
    pub fn insert(guild_id: impl Into<String>, shard_id: u32) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns(vec![Guild::Enabled, Guild::ShardId, Guild::LastUpdated])
            .to_owned();
        sea_query::Query::insert()
            .into_table(Guild::Table)
            .columns(vec![Guild::Id, Guild::Enabled, Guild::ShardId])
            .values_panic(vec![
                guild_id.into().into(),
                1.into(), // Enabled by default
                shard_id.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn toggle(guild_id: impl Into<String>, new_status: bool) -> UpdateStatement {
        let value = if new_status { 1 } else { 0 };
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::Enabled, value)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn set_shard(guild_id: impl Into<String>, shard_id: u32) -> UpdateStatement {
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::ShardId, shard_id)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn update(guild_id: impl Into<String>) -> UpdateStatement {
        let new_date = chrono::Utc::now().to_rfc3339();
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::LastUpdated, new_date)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn disable(guild_id: impl Into<String>) -> UpdateStatement {
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::Enabled, 0)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn delete(guild_id: impl Into<String>) -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .returning_all()
            .to_owned()
    }

    pub fn get_all() -> sea_query::SelectStatement {
        sea_query::Query::select().from(Guild::Table).to_owned()
    }

    pub fn get_by_id(guild_id: impl Into<String>) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .to_owned()
    }

    pub fn get_by_ids(guilds: Vec<impl Into<String>>) -> sea_query::SelectStatement {
        let guild_ids: Vec<String> = guilds.into_iter().map(|g| g.into()).collect();
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).is_in(guild_ids))
            .to_owned()
    }

    pub fn get_shard(guild_id: impl Into<String>) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id.into()))
            .columns(vec![Guild::ShardId])
            .to_owned()
    }
}
