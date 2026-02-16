use sea_query::{DeleteStatement, Iden, InsertStatement, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedMessagesSchema {
    pub id: i32, // AUTOINCREMENT
    pub guild_id: String,
    pub channel_id: String,
    pub message: String,
    pub interval: String, // e.g., 'daily', 'weekly', 'monthly'
    pub send_at: String,  // TIMESTAMP
    pub created_at: String,
}

#[derive(Iden)]
pub enum TimedMessages {
    #[iden = "timed_messages"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "message"]
    Message,
    #[iden = "interval"]
    Interval,
    #[iden = "send_at"]
    SendAt,
    #[iden = "created_at"]
    CreatedAt,
}

impl TimedMessagesSchema {
    pub fn insert(
        guild_id: &String,
        channel_id: &String,
        message: &String,
        interval: &String,
        send_at: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(TimedMessages::Table)
            .columns(vec![
                TimedMessages::GuildId,
                TimedMessages::ChannelId,
                TimedMessages::Message,
                TimedMessages::Interval,
                TimedMessages::SendAt,
                TimedMessages::CreatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                channel_id.clone().into(),
                message.clone().into(),
                interval.clone().into(),
                send_at.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn update_next_send_time(id: i32, next_send_at: &String) -> UpdateStatement {
        Query::update()
            .table(TimedMessages::Table)
            .values(vec![(TimedMessages::SendAt, next_send_at.clone().into())])
            .and_where(sea_query::Expr::col(TimedMessages::Id).eq(id))
            .to_owned()
    }

    pub fn delete_timed_message(id: i32) -> DeleteStatement {
        Query::delete()
            .from_table(TimedMessages::Table)
            .and_where(sea_query::Expr::col(TimedMessages::Id).eq(id))
            .to_owned()
    }

    pub fn delete_all_guild_messages(guild_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(TimedMessages::Table)
            .and_where(sea_query::Expr::col(TimedMessages::GuildId).eq(guild_id.clone()))
            .to_owned()
    }
}
