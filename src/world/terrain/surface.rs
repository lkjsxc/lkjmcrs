use super::fields::TerrainFields;

pub(in crate::world) fn surface_height(fields: TerrainFields) -> i32 {
    let land = fields.land.max(-0.24);
    let mountain = ((fields.ridge - 0.50) * 60.0).max(0.0);
    let rolling = land * 28.0 + fields.erosion * 9.0 + fields.detail * 4.5;
    let coast_softening = if fields.land < -0.05 {
        -((-0.05 - fields.land).min(0.35) * 8.0)
    } else {
        0.0
    };
    (78.0 + rolling + mountain + coast_softening)
        .round()
        .clamp(54.0, 136.0) as i32
}
