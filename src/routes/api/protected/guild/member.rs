use axum::{Extension, Json, Router, extract::Path, routing::post};
use reqwest::StatusCode;
use tracing::{debug, error};

use crate::{
    schema::guild::MemberSchema,
    state::{
        database::{Database, DatabaseExt},
        user::RequestedUser,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/", post(add_members).delete(delete_members))
        .route("/{user_id}", post(add_member).delete(delete_member))
}

#[worker::send]
#[axum::debug_handler]
async fn add_members(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(members): Json<Vec<String>>,
) -> Result<(), (StatusCode, String)> {
    debug!("Adding member to guild {}", guild_id);
    requested_user.bot_protection("Add Guild Member")?;
    let insert_statement = MemberSchema::insert_many(&guild_id, &members);

    let _: () = (database.execute(insert_statement).await).map_err(|e| {
        error!("Failed to add member to guild: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to add member to guild".to_string(),
        )
    })?;

    Ok(())
}

#[worker::send]
#[axum::debug_handler]
async fn add_member(
    Path((guild_id, member_id)): Path<(String, String)>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    debug!("Adding member to guild {}", guild_id);
    requested_user.bot_protection("Add Guild Member")?;
    let insert_statement = MemberSchema::insert(&guild_id, &member_id);

    let _: () = (database.execute(insert_statement).await).map_err(|e| {
        error!("Failed to add member to guild: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to add member to guild".to_string(),
        )
    })?;

    Ok(())
}

#[worker::send]
#[axum::debug_handler]
async fn delete_member(
    Path((guild_id, member_id)): Path<(String, String)>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
) -> Result<(), (StatusCode, String)> {
    debug!("Deleting member from guild {}", guild_id);
    requested_user.bot_protection("Delete Guild Member")?;
    let delete_statement = MemberSchema::delete(&guild_id, &member_id);

    let _: () = (database.execute(delete_statement).await).map_err(|e| {
        error!("Failed to delete member from guild: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete member from guild".to_string(),
        )
    })?;

    Ok(())
}

#[worker::send]
#[axum::debug_handler]
async fn delete_members(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Extension(requested_user): Extension<RequestedUser>,
    Json(members): Json<Vec<String>>,
) -> Result<(), (StatusCode, String)> {
    debug!("Deleting members from guild {}", guild_id);
    requested_user.bot_protection("Delete Guild Member")?;
    let delete_statement = MemberSchema::delete_many(&guild_id, &members);

    let _: () = (database.execute(delete_statement).await).map_err(|e| {
        error!("Failed to delete members from guild: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete members from guild".to_string(),
        )
    })?;

    Ok(())
}
