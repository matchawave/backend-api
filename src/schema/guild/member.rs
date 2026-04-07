use sea_query::{Expr, Iden};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberSchema {
    pub user_id: String,
    pub guild_id: String,
}

#[derive(Iden)]
pub enum Members {
    #[iden = "guild_members"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "user_id"]
    UserId,
}

impl Members {
    pub fn all_columns() -> Vec<Self> {
        vec![Self::GuildId, Self::UserId]
    }
}

impl MemberSchema {
    pub fn insert(guild_id: &str, user_id: &str) -> sea_query::InsertStatement {
        let on_conflict = sea_query::OnConflict::new().do_nothing().to_owned();
        sea_query::Query::insert()
            .into_table(Members::Table)
            .on_conflict(on_conflict)
            .columns(Members::all_columns())
            .values_panic([guild_id.into(), user_id.into()])
            .to_owned()
    }

    pub fn insert_many(guild_id: &str, user_ids: &[String]) -> sea_query::InsertStatement {
        let on_conflict = sea_query::OnConflict::new().do_nothing().to_owned();
        let mut query = sea_query::Query::insert()
            .into_table(Members::Table)
            .on_conflict(on_conflict)
            .columns(Members::all_columns())
            .to_owned();
        (user_ids.iter()).for_each(|user_id| {
            query = query
                .values_panic([guild_id.into(), user_id.into()])
                .to_owned()
        });

        query.to_owned()
    }

    pub fn delete(guild_id: &str, user_id: &str) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(Members::Table)
            .and_where(Expr::col(Members::GuildId).eq(guild_id))
            .and_where(Expr::col(Members::UserId).eq(user_id))
            .to_owned()
    }

    pub fn delete_many(guild_id: &str, user_ids: &[String]) -> sea_query::DeleteStatement {
        let user_ids_list =
            (user_ids.iter().map(|id| id.into())).collect::<Vec<sea_query::Value>>();
        sea_query::Query::delete()
            .from_table(Members::Table)
            .and_where(Expr::col(Members::GuildId).eq(guild_id))
            .and_where(Expr::col(Members::UserId).is_in(user_ids_list))
            .to_owned()
    }
}
