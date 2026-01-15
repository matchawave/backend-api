use sea_query::{DeleteStatement, Expr, Iden, InsertStatement, SelectStatement, UpdateStatement};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardSchema {
    pub id: u32,                      // Shard ID
    pub status: String,               // e.g., "online", "offline"
    pub latency: Option<u32>,         // Latency in milliseconds
    pub members: u32,                 // Count of members across all guilds in this shard
    pub last_updated: Option<String>, // Timestamp of the last update
    pub started_at: Option<String>,   // Timestamp when the shard started
}

#[derive(Iden)]
pub enum Shards {
    #[iden = "shards"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "status"]
    Status,
    #[iden = "latency"]
    Latency,
    #[iden = "members"]
    Members,
    #[iden = "last_updated"]
    LastUpdated,
    #[iden = "started_at"]
    StartedAt,
}

impl ShardSchema {
    pub fn new_schema(
        id: u32,
        status: String,
        latency: Option<u32>,
        members: u32,
        last_updated: Option<String>,
        started_at: Option<String>,
    ) -> InsertStatement {
        sea_query::Query::insert()
            .into_table(Shards::Table)
            .columns(vec![
                Shards::Id,
                Shards::Status,
                Shards::Latency,
                Shards::Members,
                Shards::LastUpdated,
                Shards::StartedAt,
            ])
            .values_panic(vec![
                id.into(),
                status.into(),
                latency.into(),
                members.into(),
                last_updated.into(),
                started_at.into(),
            ])
            .to_owned()
    }

    pub fn get_all() -> SelectStatement {
        sea_query::Query::select().from(Shards::Table).to_owned()
    }

    pub fn get_by_id(id: u32) -> SelectStatement {
        sea_query::Query::select()
            .from(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .to_owned()
    }

    pub fn get_by_guild(id: String) -> SelectStatement {
        sea_query::Query::select()
            .from(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .to_owned()
    }

    pub fn update_status(id: u32, status: String, latency: Option<u32>) -> UpdateStatement {
        sea_query::Query::update()
            .table(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .to_owned()
    }

    pub fn delete_by_id(id: u32) -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .to_owned()
    }
}
