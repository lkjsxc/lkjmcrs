use super::noise;

#[derive(Debug, Clone, Copy)]
pub(in crate::world) struct TerrainFields {
    pub land: f64,
    pub erosion: f64,
    pub ridge: f64,
    pub detail: f64,
    pub temperature: f64,
    pub moisture: f64,
}

pub(in crate::world) fn sample_fields(seed: i64, x: i32, z: i32) -> TerrainFields {
    let warp_x = (noise::fbm(seed ^ 0x1337, x, z, 128, 3) * 28.0).round() as i32;
    let warp_z = (noise::fbm(seed ^ 0x7331, x, z, 128, 3) * 28.0).round() as i32;
    let wx = x + warp_x;
    let wz = z + warp_z;
    TerrainFields {
        land: noise::fbm(seed ^ 0x11, wx, wz, 384, 4),
        erosion: noise::fbm(seed ^ 0x22, wx, wz, 96, 3),
        ridge: noise::ridge(seed ^ 0x33, wx, wz, 192),
        detail: noise::fbm(seed ^ 0x44, x, z, 32, 3),
        temperature: noise::fbm(seed ^ 0x5a17, wx, wz, 256, 3),
        moisture: noise::fbm(seed ^ 0x7c31, wx, wz, 256, 3),
    }
}
