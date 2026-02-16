use sea_query::{DeleteStatement, Iden, InsertStatement, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemindersSchema {
    pub id: i32, // AUTOINCREMENT
    pub user_id: String,
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub message: String,
    pub remind_at: String, // TIMESTAMP
    pub created_at: String,
}

#[derive(Iden)]
pub enum Reminders {
    #[iden = "reminders"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "message"]
    Message,
    #[iden = "remind_at"]
    RemindAt,
    #[iden = "created_at"]
    CreatedAt,
}

impl RemindersSchema {
    pub fn insert(
        user_id: &String,
        guild_id: &String,
        channel_id: Option<String>,
        message: &String,
        remind_at: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(Reminders::Table)
            .columns(vec![
                Reminders::UserId,
                Reminders::GuildId,
                Reminders::ChannelId,
                Reminders::Message,
                Reminders::RemindAt,
                Reminders::CreatedAt,
            ])
            .values_panic(vec![
                user_id.clone().into(),
                guild_id.clone().into(),
                channel_id.into(),
                message.clone().into(),
                remind_at.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn delete_reminder(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(Reminders::Table)
            .and_where(sea_query::Expr::col(Reminders::Id).eq(id))
            .to_owned()
    }

    pub fn delete_user_reminders(user_id: &String, guild_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(Reminders::Table)
            .and_where(sea_query::Expr::col(Reminders::UserId).eq(user_id.clone()))
            .and_where(sea_query::Expr::col(Reminders::GuildId).eq(guild_id.clone()))
            .to_owned()
    }
}
