use sea_query::{Iden, InsertStatement, OnConflict, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLevelsSchema {
    pub user_id: String,
    pub guild_id: String,
    pub level: i32,
    pub xp: i32,
    pub last_message_at: String,
}

#[derive(Iden)]
pub enum UserLevels {
    #[iden = "user_levels"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "level"]
    Level,
    #[iden = "xp"]
    Xp,
    #[iden = "last_message_at"]
    LastMessageAt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelConfigsSchema {
    pub guild_id: String,
    pub minimum_xp_gain: i32,
    pub maximum_xp_gain: i32,
    pub level_up_message: String,
    pub channel_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum LevelConfigs {
    #[iden = "level_configs"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "minimum_xp_gain"]
    MinimumXpGain,
    #[iden = "maximum_xp_gain"]
    MaximumXpGain,
    #[iden = "level_up_message"]
    LevelUpMessage,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelRolesSchema {
    pub guild_id: String,
    pub role_id: String,
    pub level: i32,
    pub stackable: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum LevelRoles {
    #[iden = "level_roles"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "role_id"]
    RoleId,
    #[iden = "level"]
    Level,
    #[iden = "stackable"]
    Stackable,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelXpMultipliersSchema {
    pub guild_id: String,
    pub multiplier: f64,
    pub role_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum LevelXpMultipliers {
    #[iden = "level_xp_multipliers"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "multiplier"]
    Multiplier,
    #[iden = "role_id"]
    RoleId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

// Implementations
impl UserLevelsSchema {
    pub fn insert_or_update(
        user_id: &String,
        guild_id: &String,
        xp_to_add: i32,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_columns(vec![UserLevels::Xp, UserLevels::LastMessageAt])
            .to_owned();

        Query::insert()
            .into_table(UserLevels::Table)
            .columns(vec![
                UserLevels::UserId,
                UserLevels::GuildId,
                UserLevels::Xp,
                UserLevels::LastMessageAt,
            ])
            .values_panic(vec![
                user_id.clone().into(),
                guild_id.clone().into(),
                xp_to_add.into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn update_level(
        user_id: &String,
        guild_id: &String,
        level: i32,
        xp: i32,
    ) -> UpdateStatement {
        Query::update()
            .table(UserLevels::Table)
            .values(vec![
                (UserLevels::Level, level.into()),
                (UserLevels::Xp, xp.into()),
                (
                    UserLevels::LastMessageAt,
                    chrono::Utc::now().to_rfc3339().into(),
                ),
            ])
            .and_where(sea_query::Expr::col(UserLevels::UserId).eq(user_id.clone()))
            .and_where(sea_query::Expr::col(UserLevels::GuildId).eq(guild_id.clone()))
            .to_owned()
    }
}

impl LevelConfigsSchema {
    pub fn insert_defaults(guild_id: &String) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new().do_nothing().to_owned();

        Query::insert()
            .into_table(LevelConfigs::Table)
            .columns(vec![
                LevelConfigs::GuildId,
                LevelConfigs::MinimumXpGain,
                LevelConfigs::MaximumXpGain,
                LevelConfigs::LevelUpMessage,
                LevelConfigs::CreatedAt,
                LevelConfigs::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                15.into(),
                25.into(),
                "GGs {user}, you have reached level {level.rank}!".into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }
}

impl LevelRolesSchema {
    pub fn insert(
        guild_id: &String,
        role_id: &String,
        level: i32,
        stackable: bool,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(LevelRoles::Table)
            .columns(vec![
                LevelRoles::GuildId,
                LevelRoles::RoleId,
                LevelRoles::Level,
                LevelRoles::Stackable,
                LevelRoles::CreatedAt,
                LevelRoles::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                role_id.clone().into(),
                level.into(),
                stackable.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }
}
