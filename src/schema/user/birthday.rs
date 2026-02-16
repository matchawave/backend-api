use sea_query::{Iden, InsertStatement, OnConflict, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthdaysSchema {
    pub user_id: String,
    pub birthday: String, // DATE format YYYY-MM-DD
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum Birthdays {
    #[iden = "birthdays"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "birthday"]
    Birthday,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
impl BirthdaysSchema {
    pub fn insert_or_update(user_id: &String, birthday: &String) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_columns(vec![Birthdays::Birthday, Birthdays::UpdatedAt])
            .to_owned();

        Query::insert()
            .into_table(Birthdays::Table)
            .columns(vec![
                Birthdays::UserId,
                Birthdays::Birthday,
                Birthdays::CreatedAt,
                Birthdays::UpdatedAt,
            ])
            .values_panic(vec![
                user_id.clone().into(),
                birthday.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_birthday(user_id: &String) -> sea_query::DeleteStatement {
        Query::delete()
            .from_table(Birthdays::Table)
            .and_where(sea_query::Expr::col(Birthdays::UserId).eq(user_id.clone()))
            .to_owned()
    }
}
