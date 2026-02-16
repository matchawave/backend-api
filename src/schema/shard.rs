use sea_query::{DeleteStatement, Expr, Iden, InsertStatement, SelectStatement};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardSchema {
    pub id: u32,                    // Shard ID
    pub started_at: Option<String>, // Timestamp when the shard started
}

#[derive(Iden)]
pub enum Shards {
    #[iden = "shards"]
    Table,
    #[iden = "id"]
    Id,
    #[iden = "started_at"]
    StartedAt,
}

impl ShardSchema {
    pub fn new_schema(id: u32) -> InsertStatement {
        let on_conflict = sea_query::OnConflict::new()
            .update_column(Shards::StartedAt)
            .to_owned();
        sea_query::Query::insert()
            .into_table(Shards::Table)
            .columns(vec![Shards::Id])
            .values_panic(vec![id.into()])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn get_all() -> SelectStatement {
        sea_query::Query::select()
            .from(Shards::Table)
            .columns(vec![Shards::Id, Shards::StartedAt])
            .to_owned()
    }

    pub fn get_by_id(id: u32) -> SelectStatement {
        sea_query::Query::select()
            .from(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .to_owned()
    }

    pub fn delete_by_id(id: u32) -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Shards::Table)
            .and_where(Expr::col(Shards::Id).eq(id))
            .returning_all()
            .to_owned()
    }

    pub fn delete_all() -> DeleteStatement {
        sea_query::Query::delete()
            .from_table(Shards::Table)
            .returning_all()
            .to_owned()
    }
}
