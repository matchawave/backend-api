use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;
use worker::{Env, Response};

use crate::{durables::bot::SocketSendEvent, services::websocket::WsEnvelope};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardUpdatePayload {
    pub shard_id: u32,
    pub status: String,
    pub latency_ms: Option<u32>,
    pub members: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PingPayload {
    pub timestamp: i64,
    pub avg_ping: Option<i64>,
}

#[derive(Deserialize)]
struct ShardPingPayload {
    pub ping: PingPayload,
    pub shards: Vec<ShardUpdatePayload>,
}

pub fn handle_ping_shard(
    latency: &RefCell<Option<i64>>,
    shards: &RefCell<Vec<ShardUpdatePayload>>,
    payload: Value,
) -> Option<WsEnvelope<SocketSendEvent>> {
    match serde_json::from_value::<ShardPingPayload>(payload) {
        Ok(data) => {
            handle_shard_update(shards, data.shards);
            handle_ping(latency, data.ping)
        }
        Err(e) => {
            error!("Failed to deserialize shard update payload: {}", e);
            None
        }
    }
}

fn handle_shard_update(
    shards: &RefCell<Vec<ShardUpdatePayload>>,
    new_shards: Vec<ShardUpdatePayload>,
) {
    let mut shards = shards.borrow_mut();
    if shards.is_empty() {
        *shards = new_shards;
    } else {
        for update in new_shards.into_iter() {
            if let Some(existing) = shards.get_mut(update.shard_id as usize) {
                *existing = update;
            } else {
                shards.push(update);
            }
        }
    }
}

fn handle_ping(
    latency: &RefCell<Option<i64>>,
    payload: PingPayload,
) -> Option<WsEnvelope<SocketSendEvent>> {
    *latency.borrow_mut() = payload.avg_ping;
    Some(WsEnvelope::new(SocketSendEvent::BotPong, payload))
}

pub fn get_shards(shards: &RefCell<Vec<ShardUpdatePayload>>) -> worker::Result<Response> {
    let shards = shards.borrow();
    Response::from_json(&*shards)
}
