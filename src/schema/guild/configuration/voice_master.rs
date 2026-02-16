use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query, UpdateStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceMastersSchema {
    pub guild_id: String,
    pub master_id: String,
    pub category_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Iden)]
pub enum VoiceMasters {
    #[iden = "voice_masters"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "master_id"]
    MasterId,
    #[iden = "category_id"]
    CategoryId,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
impl VoiceMastersSchema {
    pub fn insert_or_update(
        guild_id: &String,
        master_id: &String,
        category_id: &String,
    ) -> InsertStatement {
        let current_time = chrono::Utc::now().to_rfc3339();
        let on_conflict = OnConflict::new()
            .update_columns(vec![VoiceMasters::CategoryId, VoiceMasters::UpdatedAt])
            .to_owned();

        Query::insert()
            .into_table(VoiceMasters::Table)
            .columns(vec![
                VoiceMasters::GuildId,
                VoiceMasters::MasterId,
                VoiceMasters::CategoryId,
                VoiceMasters::CreatedAt,
                VoiceMasters::UpdatedAt,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                master_id.clone().into(),
                category_id.clone().into(),
                current_time.clone().into(),
                current_time.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_master(guild_id: &String, master_id: &String) -> DeleteStatement {
        Query::delete()
            .from_table(VoiceMasters::Table)
            .and_where(sea_query::Expr::col(VoiceMasters::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(VoiceMasters::MasterId).eq(master_id.clone()))
            .to_owned()
    }
}
