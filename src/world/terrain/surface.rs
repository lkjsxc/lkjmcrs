use super::fields::TerrainFields;

pub(in crate::world) fn surface_height(fields: TerrainFields) -> i32 {
    let land = fields.land.max(-0.32);
    let mountain = ((fields.ridge - 0.48) * 54.0).max(0.0);
    let rolling = land * 32.0 + fields.erosion * 10.0 + fields.detail * 5.0;
    let coast_softening = if fields.land < -0.05 {
        (fields.land + 0.05).abs() * -18.0
    } else {
        0.0
    };
    (76.0 + rolling + mountain + coast_softening)
        .round()
        .clamp(50.0, 128.0) as i32
}
