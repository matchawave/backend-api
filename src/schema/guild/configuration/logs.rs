use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query};
use serde::{Deserialize, Serialize};

#[derive(Iden)]
pub enum GuildLogConfigs {
    #[iden = "guild_log_configs"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "log_type"]
    LogType,
    #[iden = "data"]
    Data,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "created_at"]
    CreatedAt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogTypes {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "moderation")]
    Moderation,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "emoji")]
    Emoji,
    #[serde(rename = "guild")]
    Guild,
}

pub struct GuildLogConfigsSchema<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub guild_id: String,
    pub log_type: LogTypes,
    pub data: T,
    pub created_at: String,
    pub updated_at: String,
}

impl<T> GuildLogConfigsSchema<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn insert_or_update(
        guild_id: &String,
        log_type: &String,
        data: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_columns(vec![GuildLogConfigs::Data, GuildLogConfigs::UpdatedAt])
            .to_owned();

        Query::insert()
            .into_table(GuildLogConfigs::Table)
            .columns(vec![
                GuildLogConfigs::GuildId,
                GuildLogConfigs::LogType,
                GuildLogConfigs::Data,
                GuildLogConfigs::CreatedAt,
                GuildLogConfigs::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                log_type.clone().into(),
                data.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_log_config(guild_id: &String, log_type: &String) -> DeleteStatement {
        Query::delete()
            .from_table(GuildLogConfigs::Table)
            .and_where(sea_query::Expr::col(GuildLogConfigs::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(GuildLogConfigs::LogType).eq(log_type.clone()))
            .to_owned()
    }
}

macro_rules! create_log_struct {
    ($($name:ident, { $($field:ident),* $(,)? };)*) => {
        $(
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct $name {
                $(pub $field: Option<String>),* // Each field represents a log channel ID
            }
        )*
    };
}

create_log_struct!(
    MessageLog, { edit, delete, command, bulk_delete, };
    VoiceLog, { join, leave, switch, };
    ModerationLog, { ban, unban, kick, mute, unmute, warn, timout, untimeout, };
    MemberLog, { join, leave, update, };
    ChannelLog, { create, delete, update,};
    RoleLog, { create, delete, update,};
    EmojiLog, { create, delete, update, };
    GuildLog, { update, invites };
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildIgnoreChannelsSchema {
    pub guild_id: String,
    pub channel_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum GuildIgnoreChannels {
    #[iden = "guild_ignore_channels"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildIgnoreUsersSchema {
    pub guild_id: String,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum GuildIgnoreUsers {
    #[iden = "guild_ignore_users"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "user_id"]
    UserId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl GuildIgnoreChannelsSchema {
    pub fn insert(guild_id: &String, channel_id: &String) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(GuildIgnoreChannels::Table)
            .columns(vec![
                GuildIgnoreChannels::GuildId,
                GuildIgnoreChannels::ChannelId,
                GuildIgnoreChannels::CreatedAt,
                GuildIgnoreChannels::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                channel_id.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn delete_ignore_channel(guild_id: &String, channel_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(GuildIgnoreChannels::Table)
            .and_where(sea_query::Expr::col(GuildIgnoreChannels::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(GuildIgnoreChannels::ChannelId).eq(channel_id.clone()))
            .to_owned()
    }
}

impl GuildIgnoreUsersSchema {
    pub fn insert(guild_id: &String, user_id: &String) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(GuildIgnoreUsers::Table)
            .columns(vec![
                GuildIgnoreUsers::GuildId,
                GuildIgnoreUsers::UserId,
                GuildIgnoreUsers::CreatedAt,
                GuildIgnoreUsers::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                user_id.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn delete_ignore_user(guild_id: &String, user_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(GuildIgnoreUsers::Table)
            .and_where(sea_query::Expr::col(GuildIgnoreUsers::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(GuildIgnoreUsers::UserId).eq(user_id.clone()))
            .to_owned()
    }
}
