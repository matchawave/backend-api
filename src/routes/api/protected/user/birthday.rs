use axum::{Extension, Json, extract::Path};
use reqwest::StatusCode;

use serde::Deserialize;
use tracing::error;

use crate::{
    schema::user::BirthdaySchema,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

#[derive(Debug, Deserialize)]
pub struct NewBirthday {
    month: u8,
    day: u8,
    year: Option<u16>,
}

#[worker::send]
#[axum::debug_handler]
pub async fn set(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(body): Json<NewBirthday>,
) -> Result<Json<BirthdaySchema>, (StatusCode, String)> {
    requested_user.bot_protection("Set Birthday")?;
    let insert_statement =
        BirthdaySchema::insert_or_update(&user_id, body.day, body.month, body.year);

    let result: Vec<BirthdaySchema> = (database.execute(insert_statement).await).map_err(|e| {
        error!("Failed to set birthday: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set birthday".to_string(),
        )
    })?;

    if let Some(birthday) = result.first() {
        return Ok(Json(birthday.clone()));
    }

    Err((StatusCode::NOT_FOUND, "Birthday not found".to_string()))
}

#[worker::send]
#[axum::debug_handler]
pub async fn get(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<Option<BirthdaySchema>>, (StatusCode, String)> {
    let birthday = BirthdaySchema::get_birthday(&user_id);
    let birthday: Vec<BirthdaySchema> = (database.execute(birthday).await).map_err(|e| {
        error!("Failed to get birthday: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get birthday".to_string(),
        )
    })?;

    Ok(Json(birthday.first().cloned()))
}

#[worker::send]
#[axum::debug_handler]
pub async fn delete_birthday(
    Path(user_id): Path<String>,
    Extension(database): Extension<Database>,
) -> Result<Json<Option<BirthdaySchema>>, (StatusCode, String)> {
    let delete_statement = BirthdaySchema::delete_birthday(&user_id);
    let result: Vec<BirthdaySchema> = (database.execute(delete_statement).await).map_err(|e| {
        error!("Failed to delete birthday: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete birthday".to_string(),
        )
    })?;

    if let Some(birthday) = result.first() {
        return Ok(Json(Some(birthday.clone())));
    }

    Err((StatusCode::NOT_FOUND, "Birthday not found".to_string()))
}
