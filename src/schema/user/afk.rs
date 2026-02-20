use super::super::deserialize_bool;
use sea_query::{Expr, Iden};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfkConfigSchema {
    pub user_id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub per_guild: bool,
    pub default_reason: Option<String>,
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
    #[iden = "default_reason"]
    DefaultReason,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl AfkConfigSchema {
    /// Insert with upsert behavior: if the user_id already exists, update the existing record instead of inserting a new one
    pub fn insert(
        user_id: &String,
        per_guild: &Option<bool>,
        default_reason: &Option<String>,
        current_time: &str,
    ) -> sea_query::InsertStatement {
        let per_guild = per_guild.map(|b| if b { 1 } else { 0 });

        let mut on_conflict = sea_query::OnConflict::new()
            .update_column(AfkConfig::UpdatedAt)
            .to_owned();
        if per_guild.is_some() {
            // This means there is an update to the per_guild setting, so we need to update that column as well
            on_conflict.update_column(AfkConfig::PerGuild);
        }
        if default_reason.is_some() {
            // This means there is an update to the default_reason setting, so we need to update that column as well
            on_conflict.update_column(AfkConfig::DefaultReason);
        }
        let columns = vec![
            AfkConfig::UserId,
            AfkConfig::PerGuild,
            AfkConfig::DefaultReason,
            AfkConfig::CreatedAt,
            AfkConfig::UpdatedAt,
        ];
        let time = Expr::value(current_time.to_string());
        sea_query::Query::insert()
            .into_table(AfkConfig::Table)
            .on_conflict(on_conflict.to_owned())
            .columns(columns)
            .values_panic(vec![
                user_id.into(),
                per_guild.unwrap_or(0).into(),
                Expr::value(default_reason.clone()),
                time.clone(),
                time,
            ])
            .returning_all()
            .to_owned()
    }

    pub fn delete(user_id: String) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(AfkConfig::Table)
            .and_where(Expr::col(AfkConfig::UserId).eq(user_id))
            .to_owned()
    }

    pub fn get(user_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .columns(vec![
                AfkConfig::UserId,
                AfkConfig::PerGuild,
                AfkConfig::DefaultReason,
                AfkConfig::CreatedAt,
                AfkConfig::UpdatedAt,
            ])
            .from(AfkConfig::Table)
            .and_where(Expr::col(AfkConfig::UserId).eq(user_id.to_string()))
            .to_owned()
    }
}
