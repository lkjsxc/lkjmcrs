pub const HANDSHAKE: i32 = 0x00;

pub mod status {
    pub const REQUEST: i32 = 0x00;
    pub const RESPONSE: i32 = 0x00;
    pub const PING: i32 = 0x01;
    pub const PONG: i32 = 0x01;
}

pub mod login {
    pub const START: i32 = 0x00;
    pub const DISCONNECT: i32 = 0x00;
    pub const SUCCESS: i32 = 0x02;
    pub const ACKNOWLEDGED: i32 = 0x03;
}

pub mod config {
    pub const FINISH: i32 = 0x03;
}

pub mod play {
    pub const READY: i32 = 0x24;
    pub const KEEPALIVE: i32 = 0x26;
}
