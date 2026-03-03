use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;

use tracing::debug;

use crate::{
    schema::BirthdaySchema,
    services::streaming::setup_stream,
    state::{database::Database, user::RequestedUser},
};

#[worker::send]
#[axum::debug_handler]
pub async fn get_all_birthdays(
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    requested_user.bot_protection("Get All Birthdays")?;
    debug!("Fetching all birthdays from the database");
    setup_stream::<BirthdaySchema>("Birthday", database)
}
