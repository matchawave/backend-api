mod afk;
mod guild;
mod shard;
mod user;

pub use afk::*;
pub use guild::*;
pub use shard::*;
pub use user::*;

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: u8 = serde::Deserialize::deserialize(deserializer)?;
    Ok(v != 0) // Convert 0 to false, any other value to true
}
