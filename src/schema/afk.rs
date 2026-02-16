use sea_query::{Expr, Iden};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AfkStatusSchema {
    pub user_id: String,
    pub guild_id: Option<String>,
    pub reason: String,
    pub created_at: String,
}

#[derive(Iden)]
pub enum AfkStatus {
    #[iden = "afk_status"]
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
    pub fn insert<'a>(
        user_id: &'a str,
        guild_id: &'a Option<String>,
        reason: &'a str,
        current_time: &'a str,
    ) -> sea_query::InsertStatement {
        sea_query::Query::insert()
            .into_table(AfkStatus::Table)
            .columns(vec![
                AfkStatus::UserId,
                AfkStatus::GuildId,
                AfkStatus::Reason,
                AfkStatus::CreatedAt,
            ])
            .values_panic(vec![
                user_id.into(),
                guild_id.clone().into(),
                reason.into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn get(user_id: &str) -> sea_query::SelectStatement {
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
