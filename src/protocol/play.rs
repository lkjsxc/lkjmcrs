use crate::protocol::codec;

const OVERWORLD: &str = "minecraft:overworld";
const TELEPORT_ID: i32 = 1;
const DEFAULT_CHUNK_RADIUS: i32 = 2;
const START_WAITING_FOR_LEVEL_CHUNKS: u8 = 13;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bootstrap {
    pub entity_id: i32,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub player_x: f64,
    pub player_y: f64,
    pub player_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub game_mode: i8,
    pub ability_flags: i8,
}

impl Bootstrap {
    pub fn new(max_players: usize) -> Self {
        Self {
            entity_id: 1,
            max_players: max_players as i32,
            view_distance: DEFAULT_CHUNK_RADIUS,
            simulation_distance: DEFAULT_CHUNK_RADIUS,
            spawn_x: 0,
            spawn_y: 80,
            spawn_z: 0,
            player_x: 0.5,
            player_y: 80.0,
            player_z: 0.5,
            yaw: 0.0,
            pitch: 0.0,
            chunk_x: 0,
            chunk_z: 0,
            game_mode: 1,
            ability_flags: 0x0d,
        }
    }

    pub fn with_player_state(
        mut self,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        game_mode: i8,
        ability_flags: i8,
    ) -> Self {
        self.player_x = x;
        self.player_y = y;
        self.player_z = z;
        self.yaw = yaw;
        self.pitch = pitch;
        self.spawn_x = block_coord(x);
        self.spawn_y = block_coord(y);
        self.spawn_z = block_coord(z);
        self.chunk_x = self.spawn_x.div_euclid(16);
        self.chunk_z = self.spawn_z.div_euclid(16);
        self.game_mode = game_mode;
        self.ability_flags = ability_flags;
        self
    }

    pub const fn teleport_id(self) -> i32 {
        TELEPORT_ID
    }

    pub fn chunk_count(self) -> usize {
        chunk_count_for_radius(self.view_distance)
    }
}

pub fn chunk_count_for_radius(radius: i32) -> usize {
    assert!(radius >= 0, "chunk radius must be non-negative");
    let width = radius as usize * 2 + 1;
    width * width
}

pub fn encode_login(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i32(&mut out, bootstrap.entity_id);
    codec::write_bool(&mut out, false);
    codec::write_var_i32(&mut out, 1);
    codec::write_string(&mut out, OVERWORLD);
    codec::write_var_i32(&mut out, bootstrap.max_players);
    codec::write_var_i32(&mut out, bootstrap.view_distance);
    codec::write_var_i32(&mut out, bootstrap.simulation_distance);
    codec::write_bool(&mut out, false);
    codec::write_bool(&mut out, true);
    codec::write_bool(&mut out, false);
    encode_spawn_info(&mut out, bootstrap);
    codec::write_bool(&mut out, false);
    out
}

pub fn encode_chunk_cache_center(chunk_x: i32, chunk_z: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, chunk_x);
    codec::write_var_i32(&mut out, chunk_z);
    out
}

pub fn encode_chunk_cache_radius(radius: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, radius);
    out
}

pub fn encode_default_spawn_position(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_string(&mut out, OVERWORLD);
    codec::write_position(
        &mut out,
        bootstrap.spawn_x,
        bootstrap.spawn_y,
        bootstrap.spawn_z,
    );
    codec::write_f32(&mut out, 0.0);
    codec::write_f32(&mut out, 0.0);
    out
}

pub fn encode_time(age: i64, time: i64) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i64(&mut out, age);
    codec::write_i64(&mut out, time);
    codec::write_bool(&mut out, true);
    out
}

pub fn encode_player_abilities() -> Vec<u8> {
    encode_player_abilities_for(Bootstrap::new(100))
}

pub fn encode_player_abilities_for(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i8(&mut out, bootstrap.ability_flags);
    codec::write_f32(&mut out, 0.05);
    codec::write_f32(&mut out, 0.1);
    out
}

pub fn encode_start_waiting_for_chunks() -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_u8(&mut out, START_WAITING_FOR_LEVEL_CHUNKS);
    codec::write_f32(&mut out, 0.0);
    out
}

pub fn encode_initial_position(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, bootstrap.teleport_id());
    codec::write_f64(&mut out, bootstrap.player_x);
    codec::write_f64(&mut out, bootstrap.player_y);
    codec::write_f64(&mut out, bootstrap.player_z);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f32(&mut out, bootstrap.yaw);
    codec::write_f32(&mut out, bootstrap.pitch);
    codec::write_u32(&mut out, 0);
    out
}

pub fn encode_keepalive(id: i64) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i64(&mut out, id);
    out
}

fn encode_spawn_info(out: &mut Vec<u8>, bootstrap: Bootstrap) {
    codec::write_var_i32(out, 0);
    codec::write_string(out, OVERWORLD);
    codec::write_i64(out, 0);
    codec::write_i8(out, bootstrap.game_mode);
    codec::write_u8(out, 255);
    codec::write_bool(out, false);
    codec::write_bool(out, true);
    codec::write_bool(out, false);
    codec::write_var_i32(out, 0);
    codec::write_var_i32(out, 63);
}

fn block_coord(value: f64) -> i32 {
    value.floor() as i32
}
