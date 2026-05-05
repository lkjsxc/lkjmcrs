pub const MINECRAFT_VERSION: &str = "1.21.11";
pub const PROTOCOL_VERSION: i32 = 774;
pub const WORLD_VERSION: i32 = 4671;
pub const DATA_PACK_VERSION: (i32, i32) = (94, 1);
pub const RESOURCE_PACK_VERSION: (i32, i32) = (75, 0);

pub mod chunk;
pub mod codec;
#[cfg(test)]
mod codec_tests;
pub mod configuration;
pub mod ids;
pub mod login;
pub mod nbt;
pub mod play;
pub mod registry;
#[cfg(test)]
mod registry_tests;
pub mod registry_values;
pub mod registry_variants;
pub mod status;
pub mod types;
