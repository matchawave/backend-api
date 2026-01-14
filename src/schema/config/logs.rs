use sea_query::Iden;
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

pub struct GuildLogConfigsData {
    pub guild_id: String,
    pub log_type: LogTypes,
    pub data: String,
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
