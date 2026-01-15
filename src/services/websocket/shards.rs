use sea_query::{OnConflict, Query, SqliteQueryBuilder};
use serde::{Deserialize, Serialize};
use worker::Env;

use crate::{schema::Shards, state::database::Database};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShardUpdatePayload {
    pub shard_id: u32,
    pub status: String,
    pub latency_ms: Option<u32>,
    pub members: u32,
}

pub async fn handle_shard_update(
    env: Env,
    payload: ShardUpdatePayload,
) -> Result<(), worker::Error> {
    // Here you can handle the shard update, e.g., log it or store it in a database

    let general_db: Database = env.d1("DB")?.into();

    let conflict_action = OnConflict::column(Shards::Id)
        .update_columns(vec![Shards::Status, Shards::Latency, Shards::Members])
        .to_owned();
    let columns = vec![Shards::Id, Shards::Status, Shards::Latency, Shards::Members];
    let values = vec![
        payload.shard_id.into(),
        payload.status.into(),
        payload.latency_ms.into(),
        payload.members.into(),
    ];

    match (Query::insert().into_table(Shards::Table))
        .columns(columns)
        .on_conflict(conflict_action)
        .values(values)
    {
        Ok(query) => {
            let query = query.returning_all().build(SqliteQueryBuilder);
            (general_db.insert(query).await).map_err(|e| {
                worker::Error::RustError(format!("Failed to update shard info: {}", e))
            })?;
            Ok(())
        }
        Err(e) => Err(worker::Error::RustError(format!(
            "Failed to build query: {}",
            e
        ))),
    }
}
