pub(super) fn fbm(seed: i64, x: i32, z: i32, scale: i32, octaves: usize) -> f64 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut total = 0.0;
    for octave in 0..octaves {
        let octave_seed = seed ^ ((octave as i64 + 1) * 0x5deece66d);
        let octave_scale = (scale >> octave).max(4);
        value += value_noise(octave_seed, x, z, octave_scale) * amplitude;
        total += amplitude;
        amplitude *= 0.5;
    }
    value / total
}

pub(super) fn ridge(seed: i64, x: i32, z: i32, scale: i32) -> f64 {
    1.0 - fbm(seed, x, z, scale, 4).abs()
}

fn value_noise(seed: i64, x: i32, z: i32, scale: i32) -> f64 {
    let x0 = x.div_euclid(scale);
    let z0 = z.div_euclid(scale);
    let xf = (x - x0 * scale) as f64 / scale as f64;
    let zf = (z - z0 * scale) as f64 / scale as f64;
    let a = hash_unit(seed, x0, z0);
    let b = hash_unit(seed, x0 + 1, z0);
    let c = hash_unit(seed, x0, z0 + 1);
    let d = hash_unit(seed, x0 + 1, z0 + 1);
    let sx = smooth(xf);
    let sz = smooth(zf);
    lerp(lerp(a, b, sx), lerp(c, d, sx), sz)
}

fn hash_unit(seed: i64, x: i32, z: i32) -> f64 {
    let mut n = seed as u64;
    n ^= (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
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
