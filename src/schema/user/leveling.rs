use sea_query::{Iden, InsertStatement, OnConflict, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLevelProfilesSchema {
    pub user_id: String,
    pub avatar_url: Option<String>,
    pub background_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum UserLevelProfiles {
    #[iden = "user_level_profiles"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "avatar_url"]
    AvatarUrl,
    #[iden = "background_url"]
    BackgroundUrl,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
