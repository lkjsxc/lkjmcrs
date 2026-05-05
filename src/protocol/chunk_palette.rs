use crate::protocol::codec;

const BLOCK_STATE_COUNT: usize = 4096;
const INDIRECT_BLOCK_BITS: u8 = 4;

pub fn write_block_states(out: &mut Vec<u8>, states: &[i32]) {
    assert_eq!(states.len(), BLOCK_STATE_COUNT);
    let palette = palette(states);
    if palette.len() == 1 {
        write_single_value(out, palette[0]);
        return;
    }
    codec::write_u8(out, INDIRECT_BLOCK_BITS);
    codec::write_var_i32(out, palette.len() as i32);
    for state in &palette {
        codec::write_var_i32(out, *state);
    }
    let indexes = states
        .iter()
        .map(|state| palette.iter().position(|item| item == state).unwrap() as u64);
    write_fixed_packed_longs(out, indexes, INDIRECT_BLOCK_BITS, BLOCK_STATE_COUNT);
}

pub fn write_single_value(out: &mut Vec<u8>, value: i32) {
    codec::write_u8(out, 0);
    codec::write_var_i32(out, value);
}

pub fn fixed_long_count(entry_count: usize, bits: u8) -> usize {
    if bits == 0 {
        return 0;
    }
    entry_count.div_ceil(values_per_long(bits))
}

fn write_fixed_packed_longs<I>(out: &mut Vec<u8>, values: I, bits: u8, count: usize)
where
    I: IntoIterator<Item = u64>,
{
    for value in fixed_packed_longs(values, bits, count) {
        codec::write_i64(out, value as i64);
    }
}

pub fn fixed_packed_longs<I>(values: I, bits: u8, count: usize) -> Vec<u64>
where
    I: IntoIterator<Item = u64>,
{
    let mut longs = vec![0_u64; fixed_long_count(count, bits)];
    let mask = (1_u64 << bits) - 1;
    let values_per_long = values_per_long(bits);
    for (index, value) in values.into_iter().take(count).enumerate() {
        let long_index = index / values_per_long;
        let offset = (index % values_per_long) * bits as usize;
        longs[long_index] |= (value & mask) << offset;
    }
    longs
}

fn values_per_long(bits: u8) -> usize {
    64 / bits as usize
}

fn palette(states: &[i32]) -> Vec<i32> {
    let mut palette = Vec::new();
    for state in states {
        if !palette.contains(state) {
            palette.push(*state);
        }
    }
    palette
}

#[cfg(test)]
mod tests {
    use super::{BLOCK_STATE_COUNT, fixed_long_count, write_block_states, write_single_value};

    #[test]
    fn single_value_container_has_no_long_array_length() {
        let mut out = Vec::new();
        write_single_value(&mut out, 85);
        assert_eq!(out, vec![0, 85]);
    }

    #[test]
    fn biome_single_value_container_has_no_long_array_length() {
        let mut out = Vec::new();
        write_single_value(&mut out, 0);
        assert_eq!(out, vec![0, 0]);
    }

    #[test]
    fn indirect_block_container_writes_fixed_raw_longs_without_length() {
        let mut states = vec![0; BLOCK_STATE_COUNT];
        states[0] = 85;
        states[16] = 1;
        let mut out = Vec::new();
        write_block_states(&mut out, &states);
        assert_eq!(out[0], 4);
        assert_eq!(out[1], 3);
        assert_eq!(out[2], 85);
        assert_eq!(out[3], 0);
        assert_eq!(out[4], 1);
        assert_eq!(out.len(), 5 + 256 * 8);
    }

    #[test]
    fn fixed_storage_lengths_match_vanilla_shapes() {
        assert_eq!(fixed_long_count(4096, 4), 256);
        assert_eq!(fixed_long_count(64, 0), 0);
    }
}
