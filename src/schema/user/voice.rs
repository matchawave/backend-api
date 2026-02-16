use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceConfigsSchema {
    pub user_id: Option<String>,
    pub guild_id: Option<String>,
    pub name: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub locked: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum VoiceConfigs {
    #[iden = "voice_configs"]
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "name"]
    Name,
    #[iden = "bitrate"]
    Bitrate,
    #[iden = "user_limit"]
    UserLimit,
    #[iden = "locked"]
    Locked,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl VoiceConfigsSchema {
    pub fn insert_or_update(
        user_id: Option<String>,
        guild_id: Option<String>,
        name: Option<String>,
        bitrate: Option<i32>,
        user_limit: Option<i32>,
        locked: Option<String>,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_columns(vec![
                VoiceConfigs::Name,
                VoiceConfigs::Bitrate,
                VoiceConfigs::UserLimit,
                VoiceConfigs::Locked,
                VoiceConfigs::UpdatedAt,
            ])
            .to_owned();

        Query::insert()
            .into_table(VoiceConfigs::Table)
            .columns(vec![
                VoiceConfigs::UserId,
                VoiceConfigs::GuildId,
                VoiceConfigs::Name,
                VoiceConfigs::Bitrate,
                VoiceConfigs::UserLimit,
                VoiceConfigs::Locked,
                VoiceConfigs::CreatedAt,
                VoiceConfigs::UpdatedAt,
            ])
            .values_panic(vec![
                user_id.into(),
                guild_id.into(),
                name.into(),
                bitrate.into(),
                user_limit.into(),
                locked.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn update_config(
        user_id: &Option<String>,
        guild_id: &Option<String>,
        name: Option<String>,
        bitrate: Option<i32>,
        user_limit: Option<i32>,
        locked: Option<String>,
    ) -> UpdateStatement {
        let mut query = Query::update();
        query.table(VoiceConfigs::Table);

        if let Some(name) = name {
            query.value(VoiceConfigs::Name, name);
        }
        if let Some(bitrate) = bitrate {
            query.value(VoiceConfigs::Bitrate, bitrate);
        }
        if let Some(user_limit) = user_limit {
            query.value(VoiceConfigs::UserLimit, user_limit);
        }
        if let Some(locked) = locked {
            query.value(VoiceConfigs::Locked, locked);
        }

        query.value(VoiceConfigs::UpdatedAt, chrono::Utc::now().to_rfc3339());

        if let Some(user_id) = user_id {
            query.and_where(sea_query::Expr::col(VoiceConfigs::UserId).eq(user_id.clone()));
        } else {
            query.and_where(sea_query::Expr::col(VoiceConfigs::UserId).is_null());
        }

        if let Some(guild_id) = guild_id {
            query.and_where(sea_query::Expr::col(VoiceConfigs::GuildId).eq(guild_id.clone()));
        } else {
            query.and_where(sea_query::Expr::col(VoiceConfigs::GuildId).is_null());
        }

        query.to_owned()
    }
}
