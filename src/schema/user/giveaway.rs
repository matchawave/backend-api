use super::super::deserialize_bool;
use sea_query::{DeleteStatement, Iden, InsertStatement, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GiveawayEntriesSchema {
    pub giveaway_id: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Iden)]
pub enum GiveawayEntries {
    #[iden = "giveaway_entries"]
    Table,
    #[iden = "giveaway_id"]
    GiveawayId,
    #[iden = "user_id"]
    UserId,
    #[iden = "created_at"]
    CreatedAt,
}

impl GiveawayEntriesSchema {
    pub fn insert(giveaway_id: &String, user_id: &String) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(GiveawayEntries::Table)
            .columns(vec![
                GiveawayEntries::GiveawayId,
                GiveawayEntries::UserId,
                GiveawayEntries::CreatedAt,
            ])
            .values_panic(vec![
                giveaway_id.clone().into(),
                user_id.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn delete_entry(giveaway_id: &String, user_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(GiveawayEntries::Table)
            .and_where(sea_query::Expr::col(GiveawayEntries::GiveawayId).eq(giveaway_id.clone()))
            .and_where(sea_query::Expr::col(GiveawayEntries::UserId).eq(user_id.clone()))
            .to_owned()
    }

    pub fn delete_all_entries(giveaway_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(GiveawayEntries::Table)
            .and_where(sea_query::Expr::col(GiveawayEntries::GiveawayId).eq(giveaway_id.clone()))
            .to_owned()
    }
}
