use serde::{Deserialize, Serialize};
use tracing::error;
use worker::{console_log, Env};

use crate::{
    schema::ShardSchema,
    state::database::{Database, DatabaseExt},
};

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
    let general_db: Database = env.d1("DB")?.into();

    let shard_id = payload.shard_id;
    let status = payload.status;
    let latency = payload.latency_ms;
    let members = payload.members;

    if let Err(e) = general_db
        .execute(ShardSchema::update_status(
            shard_id, status, latency, members,
        ))
        .await
    {
        error!("Failed to update shard {}: {}", shard_id, e);
    }

    Ok(())
}
