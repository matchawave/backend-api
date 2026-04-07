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
    #[iden = "created_at"]
    CreatedAt,
}

impl UserSchema {}
