use sea_query::Iden;

#[derive(Iden)]
pub enum VoiceConfigs {
    #[iden = "voice_configs"]
    Table,
    #[iden = "guild_id"]
    GuildId,
    #[iden = "masters"]
    Masters,
    #[iden = "name"]
    Name,
    #[iden = "bitrate"]
    Bitrate,
    #[iden = "user_limit"]
    UserLimit,
    #[iden = "locked"]
    Locked,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
