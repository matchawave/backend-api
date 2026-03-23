use super::super::deserialize_bool;
use sea_query::{
    Alias, CommonTableExpression, Cond, Expr, Iden, InsertStatement, OnConflict, Query,
    SelectStatement,
};
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

#[derive(Iden, Clone, Copy)]
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
    pub fn get_and_insert(
        user_id: &String,
        per_guild: &Option<bool>,
        default_reason: &Option<String>,
    ) -> InsertStatement {
        let per_guild = per_guild.map(|b| if b { 1 } else { 0 });

        let mut on_conflict = OnConflict::new()
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

        let columns = [
            AfkConfig::UserId,
            AfkConfig::PerGuild,
            AfkConfig::DefaultReason,
        ];

        let config_name = Alias::new("old_cfg");

        // Get the old config for the user
        let select_query = SelectStatement::new()
            .columns(columns)
            .from(AfkConfig::Table)
            .and_where(Expr::col(AfkConfig::UserId).eq(user_id.to_string()))
            .to_owned();

        let cte = CommonTableExpression::new()
            .table_name(config_name.clone())
            .columns(columns)
            .query(select_query)
            .to_owned();

        let mut base_branch = Query::select()
            .expr(Expr::value(user_id.to_string()))
            .expr(per_guild.unwrap_or(0))
            .expr(Expr::value(default_reason.clone()))
            .to_owned();

        let fallback_branch = base_branch
            .clone()
            .cond_where(
                Cond::all().not().add(Expr::exists(
                    Query::select()
                        .expr(Expr::value(1))
                        .from(config_name.clone())
                        .to_owned(),
                )),
            )
            .clone();

        let values_select = base_branch
            .from(config_name.clone())
            .union(sea_query::UnionType::All, fallback_branch)
            .to_owned();

        // Insert the new config with upsert behavior
        match InsertStatement::new()
            .with_cte(cte)
            .into_table(AfkConfig::Table)
            .columns(columns)
            .select_from(values_select)
        {
            Ok(insert) => insert
                .on_conflict(on_conflict.to_owned())
                .returning_all()
                .to_owned(),
            Err(e) => {
                panic!("Failed to build insert statement: {e}");
            }
        }
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
