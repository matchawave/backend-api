use sea_query::{Expr, Iden, SimpleExpr};
use serde::{Deserialize, Serialize};

use crate::services::streaming::StreamableSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AfkStatusSchema {
    pub user_id: String,
    pub guild_id: Option<String>,
    pub reason: String,
    pub created_at: String,
}

#[derive(Iden)]
pub enum AfkStatus {
    #[iden = "afk_statuses"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "reason"]
    Reason,
    #[iden = "created_at"]
    CreatedAt,
}

impl AfkStatusSchema {
    pub fn insert(
        user_id: impl Into<String>,
        guild_id: &Option<String>,
        reason: impl Into<String>,
    ) -> sea_query::InsertStatement {
        // let guild_id = guild_id.map(|g| SimpleExpr::);
        sea_query::Query::insert()
            .into_table(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .values_panic(vec![
                user_id.into().into(),
                Expr::value(guild_id.clone()),
                reason.into().into(),
            ])
            .to_owned()
    }

    pub fn get(user_id: impl Into<String>) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .and_where(Expr::col(AfkStatus::UserId).eq(user_id))
            .to_owned()
    }

    pub fn all() -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .to_owned()
    }

    pub fn all_by_batch(batch_size: u64, offset: u64) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .limit(batch_size)
            .offset(offset)
            .to_owned()
    }

    pub fn get_guild(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .and_where(Expr::col(AfkStatus::GuildId).eq(guild_id))
            .to_owned()
    }

    pub fn delete(user_id: &str, guild_id: &Option<String>) -> sea_query::DeleteStatement {
        let mut query = sea_query::Query::delete();
        query
            .from_table(AfkStatus::Table)
            .and_where(Expr::col(AfkStatus::UserId).eq(user_id));
        if let Some(guild_id) = guild_id {
            query.and_where(Expr::col(AfkStatus::GuildId).eq(guild_id));
        }
        query.returning_all().to_owned()
    }
}

impl StreamableSchema for AfkStatusSchema {
    fn all_by_batch(batch_size: u64, offset: u64) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .from(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .limit(batch_size)
            .offset(offset)
            .to_owned()
    }
}
