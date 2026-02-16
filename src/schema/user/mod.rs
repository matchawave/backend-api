use sea_query::Iden;
use serde::{Deserialize, Serialize};

mod afk;
mod birthday;
mod giveaway;
mod leveling;
mod voice;
pub use afk::*;
pub use birthday::*;
pub use giveaway::*;
pub use leveling::*;
pub use voice::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSchema {
    pub id: String,
    pub guild_id: String,
    pub created_at: String,
}

#[derive(Iden)]
pub enum User {
    #[iden = "users"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "created_at"]
    CreatedAt,
}

impl UserSchema {
    pub fn insert_if_not_exists(user_id: &String, guild_id: &String) -> sea_query::InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = sea_query::OnConflict::new().do_nothing().to_owned();
        sea_query::Query::insert()
            .into_table(User::Table)
            .columns(vec![User::Id, User::GuildId, User::CreatedAt])
            .values_panic(vec![
                user_id.clone().into(),
                guild_id.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }
}
