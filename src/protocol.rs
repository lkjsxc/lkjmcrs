pub const MINECRAFT_VERSION: &str = "1.21.11";
pub const PROTOCOL_VERSION: i32 = 774;
pub const WORLD_VERSION: i32 = 4671;
pub const DATA_PACK_VERSION: (i32, i32) = (94, 1);
pub const RESOURCE_PACK_VERSION: (i32, i32) = (75, 0);

pub mod block_interaction;
pub mod chunk;
mod chunk_palette;
#[cfg(test)]
mod chunk_tests;
pub mod codec;
#[cfg(test)]
mod codec_tests;
pub mod configuration;
pub mod ids;
pub mod login;
pub mod movement;
pub mod nbt;
pub mod play;
#[cfg(test)]
mod play_tests;
pub mod registry;
mod registry_damage;
#[cfg(test)]
mod registry_damage_tests;
#[cfg(test)]
mod registry_tests;
pub mod registry_values;
pub mod registry_variants;
mod registry_world;
pub mod status;
pub mod types;
