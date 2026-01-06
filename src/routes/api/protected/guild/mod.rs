use crate::{
    schema::{GuildData, Guilds},
    state::database::Databases,
};
use axum::{
    extract::Path, http::Response, response::IntoResponse, routing::get, Extension, Json, Router,
};
use reqwest::StatusCode;
use sea_query::{Expr, OnConflict, Query, SqliteQueryBuilder};
use tracing::{error, warn};

mod settings;
// ! FOR GUILDS 0 if FALSE, 1 if TRUE
// ! This is used to enable or disable features for a guild

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_guild).post(create_new_guild).delete(delete_guild),
        )
        .nest("/settings", settings::router())
}

#[axum::debug_handler]
#[worker::send]
async fn get_guild(
    Path(id): Path<String>,
    Extension(databases): Extension<Databases>,
) -> Result<impl IntoResponse, StatusCode> {
    let query = Query::select()
        .column(Guilds::Id)
        .column(Guilds::Enabled)
        .from(Guilds::Table)
        .and_where(Expr::col(Guilds::Id).eq(id.clone()))
        .build(SqliteQueryBuilder);

    let guild = databases.general.exec_returning::<GuildData>(query).await?;
    if guild.is_empty() {
        warn!("Guild with ID {} not found", id);
        return Ok(StatusCode::OK.into_response());
    }
    if guild.len() > 1 {
        error!("Multiple guilds found with ID {}", id);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok((StatusCode::OK, Json(guild[0].to_owned())).into_response())
}

#[worker::send]
async fn create_new_guild(
    Path(id): Path<String>,
    Extension(databases): Extension<Databases>,
) -> Result<(), StatusCode> {
    let query = Query::insert()
        .into_table(Guilds::Table)
        .columns(vec![Guilds::Id, Guilds::Enabled])
        .on_conflict(
            OnConflict::column(Guilds::Id)
                .value(Guilds::Enabled, Expr::value(1))
                .to_owned(),
        )
        .values(vec![Expr::value(id), Expr::value(1)])
        .unwrap()
        .build(SqliteQueryBuilder);

    databases.general.exec(query).await
}

#[worker::send]
async fn delete_guild(
    Path(id): Path<String>,
    Extension(databases): Extension<Databases>,
) -> Result<(), StatusCode> {
    let query = Query::update()
        .table(Guilds::Table)
        .and_where(Expr::col(Guilds::Id).eq(id))
        .value(Guilds::Enabled, Expr::value(0))
        .build(SqliteQueryBuilder);
    databases.general.exec(query).await?;
    Ok(())
}
