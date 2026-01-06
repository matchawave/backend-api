use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PingPayload {
    pub timestamp: i64,
    pub avg_ping: Option<i64>,
}
