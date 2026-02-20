use std::io::Error;

use axum::{
    body::{Body, Bytes},
    http::Response,
    response::IntoResponse,
    Extension,
};
use reqwest::StatusCode;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, warn};
use wasm_bindgen_futures::spawn_local;

use crate::{
    schema::AfkStatusSchema,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

const BATCH_SIZE: usize = 100;

#[worker::send]
pub async fn get_all_afk(
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    requested_user.bot_protection("Get All AFK Statuses")?;
    debug!("Fetching all AFK statuses from the database");

    let (tx, rx) = channel::<Result<Bytes, Error>>(32);
    spawn_local(async move {
        let mut offset = 0;
        loop {
            let afk_query = AfkStatusSchema::all_by_batch(BATCH_SIZE as u64, offset);
            let users: Vec<AfkStatusSchema> = match database.execute(afk_query).await {
                Ok(users) => users,
                Err(e) => {
                    warn!("Failed to get AFK statuses: {:?}", e);
                    if let Err(e) = tx
                        .send(Err(std::io::Error::other("Failed to get AFK statuses")))
                        .await
                    {
                        debug!("Client disconnected, stopping AFK status streaming: {}", e);
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
                    debug!("Client disconnected, stopping AFK status streaming: {}", e);
                    break;
                }
            }

            offset += BATCH_SIZE as u64;
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
