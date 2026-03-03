mod stream;

use axum::{extract::Path, routing::get, Extension, Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::warn;

use crate::{
    schema::{BirthdaySchema, UserSchema},
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/{user_id}", get(get_birthday).post(set_birthday))
        .route("/all", get(stream::get_all_birthdays))
}

#[derive(Debug, Deserialize)]
pub struct NewBirthday {
    month: u8,
    day: u8,
    year: Option<u16>,
}

#[worker::send]
#[axum::debug_handler]
async fn set_birthday(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(body): Json<NewBirthday>,
) -> Result<Json<BirthdaySchema>, (StatusCode, String)> {
    requested_user.bot_protection("Set Birthday")?;
    let time = chrono::Utc::now().to_rfc3339();
    let user_query = UserSchema::insert_if_not_exists(&user_id);
    let insert_statement =
        BirthdaySchema::insert_or_update(&user_id, body.day, body.month, body.year, &time);

    let _: Vec<()> = (database.batch(vec![user_query, insert_statement]).await).map_err(|e| {
        warn!("Failed to set birthday: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set birthday".to_string(),
        )
    })?;

    Ok(Json(BirthdaySchema {
        user_id,
        day: body.day,
        month: body.month,
        year: body.year,
        created_at: time.clone(),
        updated_at: time,
    }))
}

#[worker::send]
#[axum::debug_handler]
async fn get_birthday(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<Option<BirthdaySchema>>, (StatusCode, String)> {
    let birthday = BirthdaySchema::get_birthday(&user_id);
    let birthday: Vec<BirthdaySchema> = (database.execute(birthday).await).map_err(|e| {
        warn!("Failed to get birthday: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get birthday".to_string(),
        )
    })?;

    Ok(Json(birthday.first().cloned()))
}
