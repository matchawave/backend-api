use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildMemberSchema {
    pub user_id: String,
    pub guild_id: String,
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

impl GuildMemberSchema {}
