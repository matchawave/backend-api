use axum::routing::get;
use axum::{Extension, response::IntoResponse};
use axum::{Router, middleware};
use paste::paste;
use reqwest::StatusCode;

use tracing::debug;

use crate::middleware::bot_only;

use crate::routes::api::protected::guild::SettingsBody;
use crate::schema::AfkStatusSchema;
use crate::schema::user::BirthdaySchema;
use crate::{services::streaming::setup_stream, state::database::Database};

pub fn router() -> Router {
    Router::new()
        .route("/afk", get(get_all_afks))
        .route("/birthday", get(get_all_birthdays))
        .route("/settings", get(get_all_guildsettings))
        .layer(middleware::from_fn(bot_only::middleware))
}

macro_rules! setup_stream_route {
    ($name:expr, $schema:ty) => {
        paste! {
            #[worker::send]
            #[axum::debug_handler]
            pub async fn [<get_all_ $name:lower>](
                Extension(database): Extension<Database>,
            ) -> Result<impl IntoResponse, (StatusCode, String)> {
                debug!("Fetching all {} from the database", $name);
                setup_stream::<$schema>($name, database)
            }
        }
    };
}

setup_stream_route!("AFKs", AfkStatusSchema);
setup_stream_route!("Birthdays", BirthdaySchema);
setup_stream_route!("GuildSettings", SettingsBody);
