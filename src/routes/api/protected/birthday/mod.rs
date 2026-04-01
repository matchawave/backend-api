mod stream;

use axum::{Extension, Json, Router, extract::Path, routing::get};
use chrono::Datelike;
use reqwest::StatusCode;
use sea_query::{Condition, Expr, Func, Function};
use serde::Deserialize;
use tracing::warn;

use crate::{
    schema::{BirthdaySchema, Birthdays, UserSchema},
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
    let insert_statement =
        BirthdaySchema::insert_or_update(&user_id, body.day, body.month, body.year);

    let result: Vec<BirthdaySchema> = (database.execute(insert_statement).await).map_err(|e| {
        warn!("Failed to set birthday: {:?}", e);
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

#[derive(Deserialize)]
pub struct UpcomingBirthdaysRequest {
    pub offset: u64,
    pub limit: u64,
}

#[worker::send]
#[axum::debug_handler]
async fn get_upcoming_birthdays(Path(guild_id): Path, Extension(database): Extension<Database>, Query(query): Query<UpcomingBirthdaysRequest>) -> Result<Json<Vec<BirthdaySchema>>, (StatusCode, String)> {
    let birthdays = BirthdaySchema::get_upcoming_birthdays
    let birthdays: Vec<BirthdaySchema> = (database.execute(birthdays).await).map_err(|e| {
        warn!("Failed to get upcoming birthdays: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get upcoming birthdays".to_string(),
        )
    })?;

    Ok(Json(birthdays))
}

/// Get upcoming birthdays for a specific guild
/// 
/// Get the current date, and you want to order them by month then day. For birthdays that already past, these will be also ordered by month and day, but for the next year.
///
/// It will be a joined select statement, making sure that the user is a member of the given guild.
fn get_upcoming_birthdays(guild_id: &str, offset: u64, limit: u64) -> SelectStatement {
    let bday_day_col = Expr::col((Birthdays::Table, Birthdays::Day));
    let bday_month_col = Expr::col((Birthdays::Table, Birthdays::Month));
    let bday_year_col = Expr::col((Birthdays::Table, Birthdays::Year));

    let now = chrono::Utc::now();
    let current_month = now.month();
    let current_day = now.day();

    

    let next_bday_year = Condition::any().add(
        Expr::col(bday_month_col).gt(current_month).or(Expr::col(bday_day_col).gt(current_day))
    )

    Query::select().to_owned();
        
}