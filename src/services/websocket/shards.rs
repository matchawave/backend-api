use serde::{Deserialize, Serialize};

//status
//     "connected",
//     "connecting",
//     "disconnected",
//     "handshake",
//     "identifying",
//     "resuming",
#[derive(Serialize, Deserialize)]
pub struct ShardUpdatePayload {
    pub shard_id: u32,
    pub status: String,
    pub latency_ms: Option<u128>,
    pub servers: Vec<u32>,
    pub members: u32,
}
