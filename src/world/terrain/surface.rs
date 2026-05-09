use super::noise;

pub(in crate::world) fn surface_height(seed: i64, x: i32, z: i32) -> i32 {
    let warp_x = (noise::fbm(seed ^ 0x1337, x, z, 96, 3) * 18.0).round() as i32;
    let warp_z = (noise::fbm(seed ^ 0x7331, x, z, 96, 3) * 18.0).round() as i32;
    let wx = x + warp_x;
    let wz = z + warp_z;
    let continental = noise::fbm(seed ^ 0x11, wx, wz, 192, 4);
    let erosion = noise::fbm(seed ^ 0x22, wx, wz, 64, 3);
    let ridges = noise::ridge(seed ^ 0x33, wx, wz, 128);
    let detail = noise::fbm(seed ^ 0x44, x, z, 24, 3);
    let coast = continental.max(-0.35);
    let mountain = ((ridges - 0.45) * 42.0).max(0.0);
    let rolling = coast * 24.0 + erosion * 8.0 + detail * 4.0;
    (76.0 + rolling + mountain).round().clamp(58.0, 118.0) as i32
}
