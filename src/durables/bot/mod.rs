use core::error;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};
use tracing::{debug, info};

use worker::{
    durable_object, DurableObject, Env, Method, Request, Response, Result, SetAlarmOptions, State,
    Stub, WebSocket, WebSocketIncomingMessage, WebSocketPair,
};

use crate::services::websocket::WsEnvelope;

mod alarms;
mod misc;

mod shards;

use alarms::ScheduledAlarm;

pub use shards::{PingPayload, ShardUpdatePayload};

const DURABLE_NAME: &str = "BOTROOM";

#[durable_object]
pub struct BotDurable {
    state: State,
    env: Env,
    alarms: RefCell<HashMap<i64, Vec<ScheduledAlarm>>>,
    shards: RefCell<Vec<ShardUpdatePayload>>,
    avg_latency: RefCell<Option<i64>>,
}

impl super::DurableFetch for BotDurable {
    fn fetch_object(env: &Env, name: &str) -> worker::Result<Stub> {
        let object = env.durable_object(DURABLE_NAME)?;
        object.id_from_name(name)?.get_stub()
    }
}

impl BotDurable {
    fn handle_event(&self, ws: WebSocket, envelope: WsEnvelope<SocketReceiveEvent>) -> Result<()> {
        let payload = envelope.data;
        match envelope.event {
            SocketReceiveEvent::BotUpdate => {
                if let Some(pong_message) =
                    shards::handle_ping_shard(&self.avg_latency, &self.shards, payload)
                {
                    ws.send(&pong_message)?;
                }
            }
        }
        Ok(())
    }

    fn websocket_upgrade(&self, req: Request) -> Result<(WebSocket, Option<i64>)> {
        let name = self.state.id().to_string();
        info!(
            "Handling WebSocket upgrade for BotDurable with name: {}",
            name
        );
        let Some(user_agent) = req.headers().get("User-Agent")? else {
            tracing::error!("Missing User-Agent header");
            return Err(worker::Error::Internal("Missing User-Agent header".into()));
        };

        match user_agent.split('.').collect::<Vec<&str>>().as_slice() {
            ["DiscordBot"] => {
                // initialize the alarms if this is the first connection
                let timestamp = if self.alarms.borrow().is_empty() {
                    let execute_time = chrono::Utc::now().timestamp_millis() + 60000; // 1 minute from now
                    let initial_alarms = ScheduledAlarm::initial_alarms();
                    (self.alarms.borrow_mut()).insert(execute_time, initial_alarms);
                    Some(execute_time)
                } else {
                    None
                };

                let ws = WebSocketPair::new()?;
                let server = ws.server;
                self.state
                    .accept_websocket_with_tags(&server, &[misc::BotDurableTag::Bot.as_ref()]);
                Ok((ws.client, timestamp))
            }
            _ => {
                let msg = format!("Unrecognized User-Agent: {}", user_agent);
                tracing::error!("{}", msg);
                Err(worker::Error::Internal((&msg).into()))
            }
        }
    }

    async fn handle_system_alarm(&self) -> Result<()> {
        let current_time = chrono::Utc::now().timestamp_millis();

        let mut removed_alarms = Vec::new();
        let keys: Vec<i64> = self.alarms.borrow().keys().cloned().collect();

        for &time in keys.iter() {
            if time <= current_time {
                let alarms_at_time = self.alarms.borrow().get(&time).cloned();
                if let Some(alarms_at_time) = alarms_at_time {
                    for alarm in alarms_at_time.into_iter() {
                        if let Err(e) = self.process_alarm(alarm).await {
                            tracing::error!("Failed to process alarm: {}", e);
                        }
                    }
                    removed_alarms.push(time);
                }
            }
        }

        // Remove processed alarms
        for time in removed_alarms {
            self.alarms.borrow_mut().remove(&time);
        }

        alarms::update_system_alarm(&self.state, &self.alarms).await
    }

    async fn process_alarm(&self, alarm: ScheduledAlarm) -> worker::Result<()> {
        // Placeholder for processing logic
        // This function should handle the alarm based on its type and perform necessary actions

        let result: (i64, ScheduledAlarm) = match alarm {
            _ => Err(worker::Error::Internal("Unrecognized alarm type".into())),
        }?;
        let (next_execute_time, next_alarm) = result;
        self.alarms
            .borrow_mut()
            .entry(next_execute_time)
            .or_default()
            .push(next_alarm);
        Ok(())
    }
}

impl DurableObject for BotDurable {
    fn new(state: State, env: Env) -> Self {
        let name = state.id().to_string();
        BotDurable {
            state,
            env,
            alarms: HashMap::new().into(),
            shards: Vec::new().into(),
            avg_latency: None.into(),
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        if let Ok(Some(upgrade)) = req.headers().get("Upgrade") {
            if upgrade.to_lowercase() == "websocket" {
                let (ws, timestamp) = self.websocket_upgrade(req)?;
                if let Some(timestamp) = timestamp {
                    let alarm_options = alarms::default_options();
                    (self.state.storage())
                        .set_alarm_with_options(timestamp, alarm_options)
                        .await?;
                }
                return Response::from_websocket(ws);
            }
        }
        let url = req.url()?;
        let method = req.method();
        match (method, url.path()) {
            (Method::Get, "/status") => shards::get_shards(&self.shards),
            _ => Response::error("Not Found", 404),
        }
    }

    async fn websocket_message(
        &self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> Result<()> {
        if let WebSocketIncomingMessage::String(text) = message {
            return match serde_json::from_str(&text) {
                Ok(envelope) => self.handle_event(ws, envelope),
                Err(e) => {
                    tracing::error!("Failed to parse message: {}", e);
                    Err(worker::Error::Internal("Invalid message format".into()))
                }
            };
        }
        Ok(())
    }

    async fn alarm(&self) -> Result<Response> {
        if let Err(e) = self.handle_system_alarm().await {
            tracing::error!("Failed to handle system alarm: {}", e);
        }
        Response::ok("Alarm handled")
    }

    async fn websocket_close(
        &self,
        _: worker::WebSocket,
        code: usize,
        reason: String,
        was_clean: bool,
    ) -> Result<()> {
        tracing::info!(
            "WebSocket closed: code: {}, reason: {}, cleaned: {}",
            code,
            reason,
            was_clean
        );
        Ok(())
    }

    async fn websocket_error(&self, _: WebSocket, error: worker::Error) -> Result<()> {
        tracing::error!("WebSocket error: {}", error);
        Ok(())
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
    BotUpdate,
}
