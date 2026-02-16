use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionTriggersSchema {
    pub guild_id: String,
    pub emoji: String,
    pub trigger: String,
    pub owner_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum ReactionTriggers {
    #[iden = "reaction_triggers"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "emoji"]
    Emoji,
    #[iden = "trigger"]
    Trigger,
    #[iden = "owner_id"]
    OwnerId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionChannelsSchema {
    pub guild_id: String,
    pub channel_id: String,
    pub emoji: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum ReactionChannels {
    #[iden = "reaction_channels"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "emoji"]
    Emoji,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl ReactionTriggersSchema {
    pub fn insert(
        guild_id: &String,
        emoji: &String,
        trigger: &String,
        owner_id: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(ReactionTriggers::Table)
            .columns(vec![
                ReactionTriggers::GuildId,
                ReactionTriggers::Emoji,
                ReactionTriggers::Trigger,
                ReactionTriggers::OwnerId,
                ReactionTriggers::CreatedAt,
                ReactionTriggers::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                emoji.clone().into(),
                trigger.clone().into(),
                owner_id.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn delete_trigger(guild_id: &String, trigger: &String, emoji: &String) -> DeleteStatement {
        Query::delete()
            .from_table(ReactionTriggers::Table)
            .and_where(sea_query::Expr::col(ReactionTriggers::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(ReactionTriggers::Trigger).eq(trigger.clone()))
            .and_where(sea_query::Expr::col(ReactionTriggers::Emoji).eq(emoji.clone()))
            .to_owned()
    }

    pub fn delete_all_by_owner(guild_id: &String, owner_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(ReactionTriggers::Table)
            .and_where(sea_query::Expr::col(ReactionTriggers::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(ReactionTriggers::OwnerId).eq(owner_id.clone()))
            .to_owned()
    }
}

impl ReactionChannelsSchema {
    pub fn insert_or_update(
        guild_id: &String,
        channel_id: &String,
        emoji: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_column(ReactionChannels::UpdatedAt)
            .to_owned();

        Query::insert()
            .into_table(ReactionChannels::Table)
            .columns(vec![
                ReactionChannels::GuildId,
                ReactionChannels::ChannelId,
                ReactionChannels::Emoji,
                ReactionChannels::CreatedAt,
                ReactionChannels::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                channel_id.clone().into(),
                emoji.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_channel_emoji(
        guild_id: &String,
        channel_id: &String,
        emoji: &String,
    ) -> DeleteStatement {
        Query::delete()
            .from_table(ReactionChannels::Table)
            .and_where(sea_query::Expr::col(ReactionChannels::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(ReactionChannels::ChannelId).eq(channel_id.clone()))
            .and_where(sea_query::Expr::col(ReactionChannels::Emoji).eq(emoji.clone()))
            .to_owned()
    }
}
