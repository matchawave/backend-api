use super::deserialize_bool;
use sea_query::{DeleteStatement, Iden, InsertStatement, UpdateStatement};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

mod configuration;
mod member;
mod misc;
mod permission;
mod settings;
mod utility;

pub use configuration::*;
pub use member::*;
pub use misc::*;
pub use permission::*;
pub use settings::*;
pub use utility::*;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildSchema {
    pub id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool, // Indicates if the guild is enabled (1) or disabled (0)
    pub shard_id: u32,
    pub updated_at: String,
    pub started_at: String,
}

#[derive(Iden, Debug, Clone, Copy, PartialEq, Eq)]
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
    #[iden = "updated_at"]
    UpdatedAt,
}

impl GuildSchema {
    pub fn insert(guild_id: &str, shard_id: u32) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns([Guild::Enabled, Guild::ShardId, Guild::UpdatedAt])
            .to_owned();
        sea_query::Query::insert()
            .into_table(Guild::Table)
            .columns([Guild::Id, Guild::Enabled, Guild::ShardId])
            .values_panic([
                guild_id.into(),
                1.into(), // Enabled by default
                shard_id.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn toggle(guild_id: &str, new_status: bool) -> UpdateStatement {
        let value = if new_status { 1 } else { 0 };
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::Enabled, value)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .to_owned()
    }

    pub fn set_shard(guild_id: &str, shard_id: u32) -> UpdateStatement {
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::ShardId, shard_id)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .to_owned()
    }

    pub fn update(guild_id: &str) -> UpdateStatement {
        let new_date = chrono::Utc::now().to_rfc3339();
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::UpdatedAt, new_date)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .to_owned()
    }

    pub fn disable(guild_id: &str) -> UpdateStatement {
        sea_query::Query::update()
            .table(Guild::Table)
            .value(Guild::Enabled, 0)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .to_owned()
    }

    pub fn delete(guild_id: &str) -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .returning_all()
            .to_owned()
    }

    pub fn get_all() -> sea_query::SelectStatement {
        sea_query::Query::select().from(Guild::Table).to_owned()
    }

    pub fn get_by_id(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .to_owned()
    }

    pub fn get_by_ids(guilds: &[String]) -> sea_query::SelectStatement {
        let guild_ids: Vec<String> = guilds.iter().cloned().collect();
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).is_in(guild_ids))
            .to_owned()
    }

    pub fn get_shard(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(Guild::Table)
            .and_where(sea_query::Expr::col(Guild::Id).eq(guild_id))
            .columns(vec![Guild::ShardId])
            .to_owned()
    }
}
