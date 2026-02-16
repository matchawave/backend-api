use super::super::deserialize_bool;
use sea_query::{Expr, Iden};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfkConfigSchema {
    pub user_id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub per_guild: bool,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum AfkConfig {
    #[iden = "afk_configs"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "per_guild"]
    PerGuild,
    #[iden = "message"]
    Message,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl AfkConfigSchema {
    pub fn insert(user_id: String, per_guild: bool, message: String) -> sea_query::InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let per_guild = if per_guild { 1 } else { 0 };
        let on_conflict = sea_query::OnConflict::new()
            .update_columns(vec![
                AfkConfig::PerGuild,
                AfkConfig::Message,
                AfkConfig::UpdatedAt,
            ])
            .to_owned();
        sea_query::Query::insert()
            .into_table(AfkConfig::Table)
            .columns(vec![
                AfkConfig::UserId,
                AfkConfig::PerGuild,
                AfkConfig::Message,
                AfkConfig::CreatedAt,
                AfkConfig::UpdatedAt,
            ])
            .on_conflict(on_conflict)
            .values_panic(vec![
                user_id.into(),
                per_guild.into(),
                message.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn update(user_id: String, per_guild: bool, message: String) -> sea_query::UpdateStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let per_guild = if per_guild { 1 } else { 0 };
        sea_query::Query::update()
            .table(AfkConfig::Table)
            .value(AfkConfig::PerGuild, Expr::value(per_guild))
            .value(AfkConfig::Message, Expr::value(message))
            .value(AfkConfig::UpdatedAt, Expr::value(current_time))
            .and_where(Expr::col(AfkConfig::UserId).eq(user_id))
            .to_owned()
    }

    pub fn delete(user_id: String) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(AfkConfig::Table)
            .and_where(Expr::col(AfkConfig::UserId).eq(user_id))
            .to_owned()
    }
}
