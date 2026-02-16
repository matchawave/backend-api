use sea_query::Iden;

#[derive(Iden)]
pub enum CommandAliases {
    #[iden = "command_aliases"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "command"]
    Command,
    #[iden = "alias"]
    Alias,
    #[iden = "args"]
    Arguments,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
