use strum::AsRefStr;
use worker::{Result, State};

#[derive(AsRefStr)]
pub enum BotDurableTag {
    #[strum(serialize = "bot")]
    Bot,
    #[strum(serialize = "dashboard")]
    Dashboard,
}

pub fn send_to_bot(state: State, message: &str) -> Result<()> {
    let connections = state.get_websockets_with_tag(BotDurableTag::Bot.as_ref());
    for ws in connections.iter() {
        if let Err(e) = ws.send_with_str(message) {
            tracing::error!("Failed to send message to bot: {}", e);
        }
    }
    Ok(())
}

pub fn send_to_guild(state: State, guild_id: &str, message: &str) -> Result<()> {
    let connections = state.get_websockets_with_tag(guild_id);
    for ws in connections.iter() {
        if let Err(e) = ws.send_with_str(message) {
            tracing::error!("Failed to send message to guild {}: {}", guild_id, e);
        }
    }
    Ok(())
}
