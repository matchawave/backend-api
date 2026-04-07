use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use chrono::Datelike;
use reqwest::StatusCode;
use sea_query::{Alias, Expr, SelectStatement};
use serde::Deserialize;
use tracing::error;

use crate::{
    schema::{
        guild,
        user::{BirthdaySchema, Birthdays},
    },
    state::database::{Database, DatabaseExt},
};

#[derive(Deserialize)]
pub struct UpcomingBirthdaysRequest {
    pub offset: u64,
    pub limit: u64,
}

#[worker::send]
#[axum::debug_handler]
pub async fn upcoming(
    Path(guild_id): Path<String>,
    Extension(database): Extension<Database>,
    Query(query): Query<UpcomingBirthdaysRequest>,
) -> Result<Json<Vec<BirthdaySchema>>, (StatusCode, String)> {
    let offset = query.offset;
    let limit = query.limit;
    let birthdays = get_upcoming_birthdays_query(&guild_id, offset, limit);
    let birthdays: Vec<BirthdaySchema> = (database.execute(birthdays).await).map_err(|e| {
        error!("Failed to get upcoming birthdays: {:?}", e);
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
fn get_upcoming_birthdays_query(guild_id: &str, offset: u64, limit: u64) -> SelectStatement {
    let bday_day_col = (Birthdays::Table, Birthdays::Day);
    let bday_month_col = (Birthdays::Table, Birthdays::Month);
    let guild_id_col = (guild::Members::Table, guild::Members::GuildId);
    let user_id_col = (guild::Members::Table, guild::Members::UserId);
    let bday_user_id_col = (Birthdays::Table, Birthdays::UserId);

    let now = chrono::Utc::now();
    let current_month = now.month();
    let current_day = now.day();

    // Check if the birthday has passed the current date,
    let next_bday_year = Expr::case(
        Expr::col(bday_month_col)
            .gt(current_month)
            .or(Expr::col(bday_month_col)
                .gt(current_month)
                .and(Expr::col(bday_day_col).gt(current_day))),
        Expr::cust("strftime('%Y', 'now')"), // Use the current year
    )
    .finally(Expr::cust("strftime('%Y', 'now', '+ 1 years')")); // Use the next year

    let birthday_year = Alias::new("next_bday_year");

    sea_query::Query::select()
        .columns(Birthdays::all_columns())
        .expr_as(next_bday_year, birthday_year.clone())
        .from(Birthdays::Table)
        .inner_join(
            guild::Members::Table,
            Expr::col(bday_user_id_col).equals(user_id_col),
        )
        .and_where(Expr::col(guild_id_col).eq(guild_id))
        .order_by(birthday_year, sea_query::Order::Asc)
        .order_by(bday_month_col, sea_query::Order::Asc)
        .order_by(bday_day_col, sea_query::Order::Asc)
        .limit(limit)
        .offset(offset)
        .to_owned()
}
