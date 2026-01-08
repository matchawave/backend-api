mod ping;
mod shards;

pub use ping::PingPayload;
pub use shards::{handle_shard_update, ShardUpdatePayload};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct WsEnvelope<T> {
    pub event: T,
    pub data: Value,
}

impl<T> WsEnvelope<T> {
    pub fn new<U>(event: T, data: U) -> Self
    where
        U: Serialize,
    {
        let data = serde_json::to_value(data).unwrap_or(Value::Null);
        Self { event, data }
    }

    pub fn data_as<U>(&self) -> Option<U>
    where
        U: DeserializeOwned,
    {
        serde_json::from_value(self.data.clone()).ok()
    }
}

#[derive(Hash, Eq, PartialEq, Serialize, Default)]
pub enum SocketSendEvent {
    #[default]
    Ready,
    BotPong,
}

#[derive(Hash, Eq, PartialEq, Deserialize)]
pub enum SocketReceiveEvent {
    BotPing,
    ShardUpdate,
}
