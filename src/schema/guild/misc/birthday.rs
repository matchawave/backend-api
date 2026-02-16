use sea_query::{Iden, InsertStatement, OnConflict, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthdayConfigsSchema {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum BirthdayConfigs {
    #[iden = "birthday_configs"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "message"]
    Message,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl BirthdayConfigsSchema {
    pub fn insert_or_update(
        guild_id: &String,
        channel_id: Option<String>,
        message: Option<String>,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let birthday_message = message.unwrap_or_else(|| "Happy Birthday {user}! 🎉".to_string());

        let on_conflict = OnConflict::new()
            .update_columns(vec![
                BirthdayConfigs::ChannelId,
                BirthdayConfigs::Message,
                BirthdayConfigs::UpdatedAt,
            ])
            .to_owned();

        Query::insert()
            .into_table(BirthdayConfigs::Table)
            .columns(vec![
                BirthdayConfigs::GuildId,
                BirthdayConfigs::ChannelId,
                BirthdayConfigs::Message,
                BirthdayConfigs::CreatedAt,
                BirthdayConfigs::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                channel_id.into(),
                birthday_message.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }
}
