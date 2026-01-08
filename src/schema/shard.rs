use sea_query::Iden;
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_servers<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardData {
    pub shard_id: u32,
    pub status: String,
    pub latency_ms: Option<u32>,
    #[serde(deserialize_with = "deserialize_servers")]
    pub servers: Vec<String>,
    pub members: u32,
    pub last_updated: Option<String>,
}

pub enum Shards {
    Table,
    ShardId,
    Status,
    LatencyMs,
    Servers,
    Members,
    LastUpdated,
}

impl Iden for Shards {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let value = match self {
            Shards::Table => "shards",
            Shards::ShardId => "shard_id",
            Shards::Status => "status",
            Shards::LatencyMs => "latency_ms",
            Shards::Servers => "servers",
            Shards::Members => "members",
            Shards::LastUpdated => "last_updated",
        };
        write!(s, "{value}",).unwrap();
    }
}
