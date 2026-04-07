use sea_query::{Iden, InsertStatement, Query};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PrefixSchema {
    pub guild_id: String,
    pub prefix: String,
}

#[derive(Iden, Clone, Copy)]
pub enum Prefixes {
    #[iden = "guild_prefixes"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "prefix"]
    Prefix,
}

impl PrefixSchema {
    pub fn insert(guild_id: &str, prefix: &str) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns([Prefixes::Prefix])
            .to_owned();
        Query::insert()
            .into_table(Prefixes::Table)
            .columns([Prefixes::GuildId, Prefixes::Prefix])
            .values_panic([guild_id.into(), prefix.into()])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete(guild_id: &str) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(Prefixes::Table)
            .and_where(sea_query::Expr::col(Prefixes::GuildId).eq(guild_id))
            .to_owned()
    }

    pub fn get(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .columns([Prefixes::Prefix])
            .from(Prefixes::Table)
            .and_where(sea_query::Expr::col(Prefixes::GuildId).eq(guild_id))
            .to_owned()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct LanguageSchema {
    pub guild_id: String,
    pub language: String,
}

#[derive(Iden, Clone, Copy)]
pub enum Languages {
    #[iden = "guild_languages"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "language"]
    Language,
}

impl LanguageSchema {
    pub fn insert(guild_id: &str, language: &str) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns([Languages::Language])
            .to_owned();
        Query::insert()
            .into_table(Languages::Table)
            .columns([Languages::GuildId, Languages::Language])
            .values_panic([guild_id.into(), language.into()])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete(guild_id: &str) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(Languages::Table)
            .and_where(sea_query::Expr::col(Languages::GuildId).eq(guild_id))
            .to_owned()
    }

    pub fn get(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .columns([Languages::Language])
            .from(Languages::Table)
            .and_where(sea_query::Expr::col(Languages::GuildId).eq(guild_id))
            .to_owned()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ColourSchema {
    pub guild_id: String,
    pub colour: String,
}

#[derive(Iden, Clone, Copy)]
pub enum Colours {
    #[iden = "guild_colours"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "colour"]
    Colour,
}

impl ColourSchema {
    pub fn insert(guild_id: &str, colour: &str) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns([Colours::Colour])
            .to_owned();
        Query::insert()
            .into_table(Colours::Table)
            .columns([Colours::GuildId, Colours::Colour])
            .values_panic([guild_id.into(), colour.into()])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete(guild_id: &str) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(Colours::Table)
            .and_where(sea_query::Expr::col(Colours::GuildId).eq(guild_id))
            .to_owned()
    }

    pub fn get(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .columns([Colours::Colour])
            .from(Colours::Table)
            .and_where(sea_query::Expr::col(Colours::GuildId).eq(guild_id))
            .to_owned()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TimezoneSchema {
    pub guild_id: String,
    pub timezone: String,
}

#[derive(Iden, Clone, Copy)]
pub enum Timezones {
    #[iden = "guild_timezones"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "timezone"]
    Timezone,
}

impl TimezoneSchema {
    pub fn insert(guild_id: &str, timezone: &str) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_columns([Timezones::Timezone])
            .to_owned();
        Query::insert()
            .into_table(Timezones::Table)
            .columns([Timezones::GuildId, Timezones::Timezone])
            .values_panic([guild_id.into(), timezone.into()])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete(guild_id: &str) -> sea_query::DeleteStatement {
        sea_query::Query::delete()
            .from_table(Timezones::Table)
            .and_where(sea_query::Expr::col(Timezones::GuildId).eq(guild_id))
            .to_owned()
    }

    pub fn get(guild_id: &str) -> sea_query::SelectStatement {
        sea_query::Query::select()
            .columns([Timezones::Timezone])
            .from(Timezones::Table)
            .and_where(sea_query::Expr::col(Timezones::GuildId).eq(guild_id))
            .to_owned()
    }
}
