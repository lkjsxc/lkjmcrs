use super::column::TerrainColumn;

const MIN_CAVE_Y: i32 = 8;
const SURFACE_MARGIN: i32 = 8;

pub(in crate::world) fn carves_air(
    seed: i64,
    x: i32,
    y: i32,
    z: i32,
    column: TerrainColumn,
) -> bool {
    if column.water_y.is_some() || y < MIN_CAVE_Y || y > column.surface_y - SURFACE_MARGIN {
        return false;
    }

    let broad = value_noise(seed ^ 0x6c3f, x, y, z, 64);
    let detail = fbm(seed ^ 0x3f6c, x, y, z, 32, 3);
    let pocket = fbm(seed ^ 0x51a7, x, y, z, 18, 2);
    broad * 0.45 + detail * 0.4 + pocket * 0.15 > 0.42
}

fn fbm(seed: i64, x: i32, y: i32, z: i32, scale: i32, octaves: usize) -> f64 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut total = 0.0;
    for octave in 0..octaves {
        let octave_seed = seed ^ ((octave as i64 + 1) * 0x5deece66d);
        let octave_scale = (scale >> octave).max(4);
        value += value_noise(octave_seed, x, y, z, octave_scale) * amplitude;
        total += amplitude;
        amplitude *= 0.5;
    }
    value / total
}

fn value_noise(seed: i64, x: i32, y: i32, z: i32, scale: i32) -> f64 {
    let x0 = x.div_euclid(scale);
    let y0 = y.div_euclid(scale);
    let z0 = z.div_euclid(scale);
    let xf = (x - x0 * scale) as f64 / scale as f64;
    let yf = (y - y0 * scale) as f64 / scale as f64;
    let zf = (z - z0 * scale) as f64 / scale as f64;
    let sx = smooth(xf);
    let sy = smooth(yf);
    let sz = smooth(zf);
    let x00 = lerp(
        hash_unit(seed, x0, y0, z0),
        hash_unit(seed, x0 + 1, y0, z0),
        sx,
    );
    let x10 = lerp(
        hash_unit(seed, x0, y0 + 1, z0),
        hash_unit(seed, x0 + 1, y0 + 1, z0),
        sx,
    );
    let x01 = lerp(
        hash_unit(seed, x0, y0, z0 + 1),
        hash_unit(seed, x0 + 1, y0, z0 + 1),
        sx,
    );
    let x11 = lerp(
        hash_unit(seed, x0, y0 + 1, z0 + 1),
        hash_unit(seed, x0 + 1, y0 + 1, z0 + 1),
        sx,
    );
    lerp(lerp(x00, x10, sy), lerp(x01, x11, sy), sz)
}

fn hash_unit(seed: i64, x: i32, y: i32, z: i32) -> f64 {
    let mut n = seed as u64;
    n ^= (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    n ^= (y as i64 as u64).wrapping_mul(0x94d0_49bb_1331_11eb);
    n ^= (z as i64 as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n ^= n >> 30;
    n = n.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n ^= n >> 27;
    n = n.wrapping_mul(0x94d0_49bb_1331_11eb);
    (((n ^ (n >> 31)) >> 11) as f64 / ((1_u64 << 53) as f64)) * 2.0 - 1.0
}

fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
