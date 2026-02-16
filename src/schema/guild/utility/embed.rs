use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbedsSchema {
    pub name: String,
    pub guild_id: String,
    pub content: Option<String>,
    pub embeds: String,     // JSON array as string, defaults to "[]"
    pub components: String, // JSON array as string, defaults to "[]"
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum Embeds {
    #[iden = "embeds"]
    Table,
    #[iden = "name"]
    Name,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "content"]
    Content,
    #[iden = "embeds"]
    Embeds,
    #[iden = "components"]
    Components,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

impl EmbedsSchema {
    pub fn insert_or_update(
        name: &String,
        guild_id: &String,
        content: Option<String>,
        embeds: Option<String>,
        components: Option<String>,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let embeds_json = embeds.unwrap_or_else(|| "[]".to_string());
        let components_json = components.unwrap_or_else(|| "[]".to_string());

        let on_conflict = OnConflict::new()
            .update_columns(vec![
                Embeds::Content,
                Embeds::Embeds,
                Embeds::Components,
                Embeds::UpdatedAt,
            ])
            .to_owned();

        Query::insert()
            .into_table(Embeds::Table)
            .columns(vec![
                Embeds::Name,
                Embeds::GuildId,
                Embeds::Content,
                Embeds::Embeds,
                Embeds::Components,
                Embeds::CreatedAt,
                Embeds::UpdatedAt,
            ])
            .values_panic(vec![
                name.clone().into(),
                guild_id.clone().into(),
                content.into(),
                embeds_json.into(),
                components_json.into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn update_content(
        name: &String,
        guild_id: &String,
        content: Option<String>,
    ) -> UpdateStatement {
        Query::update()
            .table(Embeds::Table)
            .values(vec![
                (Embeds::Content, content.into()),
                (Embeds::UpdatedAt, chrono::Utc::now().to_rfc3339().into()),
            ])
            .and_where(sea_query::Expr::col(Embeds::Name).eq(name.clone()))
            .and_where(sea_query::Expr::col(Embeds::GuildId).eq(guild_id.clone()))
            .to_owned()
    }

    pub fn update_embeds(name: &String, guild_id: &String, embeds: &String) -> UpdateStatement {
        Query::update()
            .table(Embeds::Table)
            .values(vec![
                (Embeds::Embeds, embeds.clone().into()),
                (Embeds::UpdatedAt, chrono::Utc::now().to_rfc3339().into()),
            ])
            .and_where(sea_query::Expr::col(Embeds::Name).eq(name.clone()))
            .and_where(sea_query::Expr::col(Embeds::GuildId).eq(guild_id.clone()))
            .to_owned()
    }

    pub fn update_components(
        name: &String,
        guild_id: &String,
        components: &String,
    ) -> UpdateStatement {
        Query::update()
            .table(Embeds::Table)
            .values(vec![
                (Embeds::Components, components.clone().into()),
                (Embeds::UpdatedAt, chrono::Utc::now().to_rfc3339().into()),
            ])
            .and_where(sea_query::Expr::col(Embeds::Name).eq(name.clone()))
            .and_where(sea_query::Expr::col(Embeds::GuildId).eq(guild_id.clone()))
            .to_owned()
    }

    pub fn delete_embed(name: &String, guild_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(Embeds::Table)
            .and_where(sea_query::Expr::col(Embeds::Name).eq(name.clone()))
            .and_where(sea_query::Expr::col(Embeds::GuildId).eq(guild_id.clone()))
            .to_owned()
    }
}
