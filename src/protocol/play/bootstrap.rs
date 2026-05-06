const TELEPORT_ID: i32 = 1;
const DEFAULT_CHUNK_RADIUS: i32 = 2;

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

    pub fn with_distances(mut self, view_distance: i32, simulation_distance: i32) -> Self {
        self.view_distance = view_distance;
        self.simulation_distance = simulation_distance;
        self
    }

    pub fn with_player_state(
        mut self,
        position: (f64, f64, f64),
        look: (f32, f32),
        mode: (i8, i8),
    ) -> Self {
        let (x, y, z) = position;
        self.player_x = x;
        self.player_y = y;
        self.player_z = z;
        self.yaw = look.0;
        self.pitch = look.1;
        self.spawn_x = block_coord(x);
        self.spawn_y = block_coord(y);
        self.spawn_z = block_coord(z);
        self.chunk_x = self.spawn_x.div_euclid(16);
        self.chunk_z = self.spawn_z.div_euclid(16);
        self.game_mode = mode.0;
        self.ability_flags = mode.1;
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

fn block_coord(value: f64) -> i32 {
    value.floor() as i32
}
