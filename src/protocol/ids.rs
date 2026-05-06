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
    pub const SERVERBOUND_SETTINGS: i32 = 0x00;
    pub const SERVERBOUND_CUSTOM_PAYLOAD: i32 = 0x02;
    pub const FINISH: i32 = 0x03;
    pub const REGISTRY_DATA: i32 = 0x07;
    pub const FEATURE_FLAGS: i32 = 0x0c;
    pub const TAGS: i32 = 0x0d;
    pub const SELECT_KNOWN_PACKS: i32 = 0x0e;
    pub const SERVERBOUND_SELECT_KNOWN_PACKS: i32 = 0x07;
}

pub mod play {
    pub const BLOCK_CHANGED_ACK: i32 = 0x04;
    pub const BLOCK_UPDATE: i32 = 0x08;
    pub const CHUNK_BATCH_FINISHED: i32 = 0x0b;
    pub const CHUNK_BATCH_START: i32 = 0x0c;
    pub const DECLARE_COMMANDS: i32 = 0x10;
    pub const KICK_DISCONNECT: i32 = 0x20;
    pub const UNLOAD_CHUNK: i32 = 0x25;
    pub const GAME_STATE_CHANGE: i32 = 0x26;
    pub const KEEPALIVE: i32 = 0x2b;
    pub const LEVEL_CHUNK_WITH_LIGHT: i32 = 0x2c;
    pub const UPDATE_LIGHT: i32 = 0x2f;
    pub const LOGIN: i32 = 0x30;
    pub const PLAYER_ABILITIES: i32 = 0x3e;
    pub const PLAYER_POSITION: i32 = 0x46;
    pub const CHUNK_CACHE_CENTER: i32 = 0x5c;
    pub const CHUNK_CACHE_RADIUS: i32 = 0x5d;
    pub const SET_CURSOR_ITEM: i32 = 0x5e;
    pub const DEFAULT_SPAWN_POSITION: i32 = 0x5f;
    pub const HELD_ITEM_SLOT: i32 = 0x67;
    pub const SET_PLAYER_INVENTORY: i32 = 0x6a;
    pub const SET_TIME: i32 = 0x6f;
    pub const SYSTEM_CHAT: i32 = 0x77;

    pub const SERVERBOUND_TELEPORT_CONFIRM: i32 = 0x00;
    pub const SERVERBOUND_CHAT_COMMAND: i32 = 0x06;
    pub const SERVERBOUND_CHAT_COMMAND_SIGNED: i32 = 0x07;
    pub const SERVERBOUND_CHAT_MESSAGE: i32 = 0x08;
    pub const SERVERBOUND_SETTINGS: i32 = 0x0d;
    pub const SERVERBOUND_KEEPALIVE: i32 = 0x1b;
    pub const SERVERBOUND_POSITION: i32 = 0x1d;
    pub const SERVERBOUND_POSITION_LOOK: i32 = 0x1e;
    pub const SERVERBOUND_LOOK: i32 = 0x1f;
    pub const SERVERBOUND_FLYING: i32 = 0x20;
    pub const SERVERBOUND_PLAYER_ACTION: i32 = 0x28;
    pub const SERVERBOUND_CHUNK_BATCH_RECEIVED: i32 = 0x0a;
    pub const SERVERBOUND_PLAYER_LOADED: i32 = 0x2b;
    pub const SERVERBOUND_PONG: i32 = 0x2c;
    pub const SERVERBOUND_HELD_ITEM_SLOT: i32 = 0x34;
    pub const SERVERBOUND_SWING: i32 = 0x3c;
    pub const SERVERBOUND_USE_ITEM_ON: i32 = 0x3f;
}

#[cfg(test)]
mod tests {
    use super::{config, login, play};

    #[test]
    fn login_packet_ids_match_protocol_774() {
        assert_eq!(login::START, 0x00);
        assert_eq!(login::SUCCESS, 0x02);
        assert_eq!(login::ACKNOWLEDGED, 0x03);
    }

    #[test]
    fn configuration_packet_ids_match_protocol_774() {
        assert_eq!(config::SERVERBOUND_SETTINGS, 0x00);
        assert_eq!(config::SERVERBOUND_CUSTOM_PAYLOAD, 0x02);
        assert_eq!(config::FINISH, 0x03);
        assert_eq!(config::REGISTRY_DATA, 0x07);
        assert_eq!(config::SERVERBOUND_SELECT_KNOWN_PACKS, 0x07);
        assert_eq!(config::FEATURE_FLAGS, 0x0c);
        assert_eq!(config::TAGS, 0x0d);
        assert_eq!(config::SELECT_KNOWN_PACKS, 0x0e);
    }

    #[test]
    fn play_packet_ids_match_protocol_774() {
        assert_eq!(play::BLOCK_CHANGED_ACK, 0x04);
        assert_eq!(play::BLOCK_UPDATE, 0x08);
        assert_eq!(play::CHUNK_BATCH_FINISHED, 0x0b);
        assert_eq!(play::CHUNK_BATCH_START, 0x0c);
        assert_eq!(play::DECLARE_COMMANDS, 0x10);
        assert_eq!(play::KICK_DISCONNECT, 0x20);
        assert_eq!(play::UNLOAD_CHUNK, 0x25);
        assert_eq!(play::GAME_STATE_CHANGE, 0x26);
        assert_eq!(play::KEEPALIVE, 0x2b);
        assert_eq!(play::LEVEL_CHUNK_WITH_LIGHT, 0x2c);
        assert_eq!(play::UPDATE_LIGHT, 0x2f);
        assert_eq!(play::LOGIN, 0x30);
        assert_eq!(play::PLAYER_ABILITIES, 0x3e);
        assert_eq!(play::PLAYER_POSITION, 0x46);
        assert_eq!(play::CHUNK_CACHE_CENTER, 0x5c);
        assert_eq!(play::CHUNK_CACHE_RADIUS, 0x5d);
        assert_eq!(play::SET_CURSOR_ITEM, 0x5e);
        assert_eq!(play::DEFAULT_SPAWN_POSITION, 0x5f);
        assert_eq!(play::HELD_ITEM_SLOT, 0x67);
        assert_eq!(play::SET_PLAYER_INVENTORY, 0x6a);
        assert_eq!(play::SET_TIME, 0x6f);
        assert_eq!(play::SYSTEM_CHAT, 0x77);
        assert_eq!(play::SERVERBOUND_TELEPORT_CONFIRM, 0x00);
        assert_eq!(play::SERVERBOUND_CHAT_COMMAND, 0x06);
        assert_eq!(play::SERVERBOUND_CHAT_COMMAND_SIGNED, 0x07);
        assert_eq!(play::SERVERBOUND_CHAT_MESSAGE, 0x08);
        assert_eq!(play::SERVERBOUND_CHUNK_BATCH_RECEIVED, 0x0a);
        assert_eq!(play::SERVERBOUND_PLAYER_ACTION, 0x28);
        assert_eq!(play::SERVERBOUND_PLAYER_LOADED, 0x2b);
        assert_eq!(play::SERVERBOUND_SWING, 0x3c);
        assert_eq!(play::SERVERBOUND_HELD_ITEM_SLOT, 0x34);
        assert_eq!(play::SERVERBOUND_USE_ITEM_ON, 0x3f);
    }

    #[test]
    fn play_packet_0x21_is_not_game_state_change_in_protocol_774() {
        assert_ne!(play::GAME_STATE_CHANGE, 0x21);
    }
}
