use chrono::Datelike;
use sea_query::{Expr, Iden, InsertStatement, OnConflict, Query, SelectStatement};
use serde::{Deserialize, Serialize};

use crate::services::streaming::StreamableSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthdaySchema {
    pub user_id: String,
    pub day: u8,
    pub month: u8,
    pub year: Option<u16>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden, Clone, Copy, Debug)]
pub enum Birthdays {
    #[iden = "birthdays"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "day"]
    Day,
    #[iden = "month"]
    Month,
    #[iden = "year"]
    Year,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl Birthdays {
    pub fn all_columns() -> Vec<Self> {
        vec![
            Self::UserId,
            Self::Day,
            Self::Month,
            Self::Year,
            Self::CreatedAt,
            Self::UpdatedAt,
        ]
    }
}

impl BirthdaySchema {
    pub fn insert_or_update(
        user_id: &str,
        day: u8,
        month: u8,
        year: Option<u16>,
    ) -> InsertStatement {
        let on_conflict = OnConflict::new()
            .update_columns(vec![
                Birthdays::Day,
                Birthdays::Month,
                Birthdays::Year,
                Birthdays::UpdatedAt,
            ])
            .to_owned();

        Query::insert()
            .into_table(Birthdays::Table)
            .columns(vec![
                Birthdays::UserId,
                Birthdays::Day,
                Birthdays::Month,
                Birthdays::Year,
            ])
            .on_conflict(on_conflict)
            .values_panic([user_id.into(), day.into(), month.into(), Expr::value(year)])
            .to_owned()
    }

    pub fn get_birthday(user_id: &str) -> SelectStatement {
        Query::select()
            .columns(vec![
                Birthdays::UserId,
                Birthdays::Day,
                Birthdays::Month,
                Birthdays::Year,
                Birthdays::CreatedAt,
                Birthdays::UpdatedAt,
            ])
            .from(Birthdays::Table)
            .and_where(sea_query::Expr::col(Birthdays::UserId).eq(user_id))
            .to_owned()
    }

    pub fn delete_birthday(user_id: &str) -> sea_query::DeleteStatement {
        Query::delete()
            .from_table(Birthdays::Table)
            .and_where(sea_query::Expr::col(Birthdays::UserId).eq(user_id))
            .returning_all()
            .to_owned()
    }
}

impl StreamableSchema for BirthdaySchema {
    fn all_by_batch(batch_size: u64, offset: u64) -> sea_query::SelectStatement {
        let now = chrono::Utc::now();
        let current_month = now.month();
        let current_day = now.day();

        Query::select()
            .columns(vec![
                Birthdays::UserId,
                Birthdays::Day,
                Birthdays::Month,
                Birthdays::Year,
                Birthdays::CreatedAt,
                Birthdays::UpdatedAt,
            ])
            .from(Birthdays::Table)
            .and_where(
                Expr::col(Birthdays::Month)
                    .eq(current_month)
                    .and(Expr::col(Birthdays::Day).eq(current_day)),
            )
            .limit(batch_size)
            .offset(offset)
            .to_owned()
    }
}
