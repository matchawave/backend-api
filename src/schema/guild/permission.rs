use sea_query::{DeleteStatement, Iden, InsertStatement, OnConflict, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionRolesSchema {
    pub guild_id: String,
    pub permission: String,
    pub role_id: Option<String>,
}

#[derive(Iden)]
pub enum PermissionRoles {
    #[iden = "permission_roles"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "permission"]
    Permission,
    #[iden = "role_id"]
    RoleId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionUsersSchema {
    pub guild_id: String,
    pub permission: String,
    pub user_id: Option<String>,
}

#[derive(Iden)]
pub enum PermissionUsers {
    #[iden = "permission_users"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "permission"]
    Permission,
    #[iden = "user_id"]
    UserId,
}

impl PermissionRolesSchema {
    pub fn insert_or_update(
        guild_id: &String,
        permission: &String,
        role_id: Option<String>,
    ) -> InsertStatement {
        let on_conflict = OnConflict::new()
            .update_column(PermissionRoles::RoleId)
            .to_owned();

        Query::insert()
            .into_table(PermissionRoles::Table)
            .columns(vec![
                PermissionRoles::GuildId,
                PermissionRoles::Permission,
                PermissionRoles::RoleId,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                permission.clone().into(),
                role_id.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_permission(guild_id: &String, permission: &String) -> DeleteStatement {
        Query::delete()
            .from_table(PermissionRoles::Table)
            .and_where(sea_query::Expr::col(PermissionRoles::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(PermissionRoles::Permission).eq(permission.clone()))
            .to_owned()
    }
}

impl PermissionUsersSchema {
    pub fn insert_or_update(
        guild_id: &String,
        permission: &String,
        user_id: Option<String>,
    ) -> InsertStatement {
        let on_conflict = OnConflict::new()
            .update_column(PermissionUsers::UserId)
            .to_owned();

        Query::insert()
            .into_table(PermissionUsers::Table)
            .columns(vec![
                PermissionUsers::GuildId,
                PermissionUsers::Permission,
                PermissionUsers::UserId,
            ])
            .values_panic(vec![
                guild_id.clone().into(),
                permission.clone().into(),
                user_id.into(),
            ])
            .on_conflict(on_conflict)
            .to_owned()
    }

    pub fn delete_permission(guild_id: &String, permission: &String) -> DeleteStatement {
        Query::delete()
            .from_table(PermissionUsers::Table)
            .and_where(sea_query::Expr::col(PermissionUsers::GuildId).eq(guild_id.clone()))
            .and_where(sea_query::Expr::col(PermissionUsers::Permission).eq(permission.clone()))
            .to_owned()
    }
}
