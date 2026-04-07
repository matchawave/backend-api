use std::io::Error;

use axum::{
    body::{Body, Bytes},
    http::Response,
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, warn};
use wasm_bindgen_futures::spawn_local;

use crate::state::database::{Database, DatabaseExt};

const BATCH_SIZE: u64 = 100;

pub trait StreamableSchema {
    fn all_by_batch(batch_size: u64, offset: u64) -> sea_query::SelectStatement;
}

pub fn setup_stream<T>(
    name: &str,
    database: Database,
) -> Result<impl IntoResponse, (StatusCode, String)>
where
    T: Serialize + for<'a> Deserialize<'a>,
    T: StreamableSchema + Send + 'static,
{
    debug!("Fetching all {name} schema from the database");
    let name = name.to_string();
    let (tx, rx) = channel::<Result<Bytes, Error>>(32);
    spawn_local(async move {
        let mut offset = 0;
        loop {
            let query = T::all_by_batch(BATCH_SIZE, offset);
            let users: Vec<T> = match database.execute(query).await {
                Ok(users) => users,
                Err(e) => {
                    warn!("Failed to get {name} schema: {:?}", e);
                    if let Err(e) = tx
                        .send(Err(std::io::Error::other(format!(
                            "Failed to get {name} schema"
                        ))))
                        .await
                    {
                        debug!("Client disconnected, stopping {name} streaming: {}", e);
                        break;
                    }
                    return;
                }
            };

            if users.is_empty() {
                break;
            }

            for user in users {
                let json = serde_json::to_string(&user).unwrap_or_else(|_| "{}".to_string());
                if let Err(e) = tx.send(Ok(Bytes::from(format!("{}\n", json)))).await {
                    debug!("Client disconnected, stopping {name} streaming: {}", e);
                    break;
                }
            }

            offset += BATCH_SIZE;
        }
    });

    let body = Body::from_stream(ReceiverStream::new(rx));

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-ndjson")
        .body(body)
        .map_err(|e| {
            warn!("Failed to build response: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build response".to_string(),
            )
        })
}
