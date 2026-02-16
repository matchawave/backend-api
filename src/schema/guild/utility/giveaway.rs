use super::super::deserialize_bool;
use sea_query::{DeleteStatement, Iden, InsertStatement, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GiveawaysSchema {
    pub giveaway_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub prize: String,
    pub end_time: String, // TIMESTAMP
    pub winners_count: i32,
    pub host_id: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ended: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum Giveaways {
    #[iden = "giveaways"]
    Table,
    #[iden = "giveaway_id"]
    GiveawayId,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "channel_id"]
    ChannelId,
    #[iden = "message_id"]
    MessageId,
    #[iden = "prize"]
    Prize,
    #[iden = "end_time"]
    EndTime,
    #[iden = "winners_count"]
    WinnersCount,
    #[iden = "host_id"]
    HostId,
    #[iden = "ended"]
    Ended,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl GiveawaysSchema {
    pub fn insert(
        giveaway_id: &String,
        guild_id: &String,
        channel_id: &String,
        message_id: &String,
        prize: &String,
        end_time: &String,
        winners_count: i32,
        host_id: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();

        Query::insert()
            .into_table(Giveaways::Table)
            .columns(vec![
                Giveaways::GiveawayId,
                Giveaways::GuildId,
                Giveaways::ChannelId,
                Giveaways::MessageId,
                Giveaways::Prize,
                Giveaways::EndTime,
                Giveaways::WinnersCount,
                Giveaways::HostId,
                Giveaways::Ended,
                Giveaways::CreatedAt,
                Giveaways::UpdatedAt,
            ])
            .values_panic(vec![
                giveaway_id.clone().into(),
                guild_id.clone().into(),
                channel_id.clone().into(),
                message_id.clone().into(),
                prize.clone().into(),
                end_time.clone().into(),
                winners_count.into(),
                host_id.clone().into(),
                false.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .to_owned()
    }

    pub fn mark_ended(giveaway_id: &String) -> UpdateStatement {
        Query::update()
            .table(Giveaways::Table)
            .values(vec![
                (Giveaways::Ended, true.into()),
                (Giveaways::UpdatedAt, chrono::Utc::now().to_rfc3339().into()),
            ])
            .and_where(sea_query::Expr::col(Giveaways::GiveawayId).eq(giveaway_id.clone()))
            .to_owned()
    }

    pub fn delete_giveaway(giveaway_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(Giveaways::Table)
            .and_where(sea_query::Expr::col(Giveaways::GiveawayId).eq(giveaway_id.clone()))
            .to_owned()
    }
}
