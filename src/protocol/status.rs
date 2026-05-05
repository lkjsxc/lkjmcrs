use crate::protocol::{MINECRAFT_VERSION, PROTOCOL_VERSION};
use serde::Serialize;

#[derive(Serialize)]
struct StatusResponse<'a> {
    version: Version<'a>,
    players: Players,
    description: Text<'a>,
    enforces_secure_chat: bool,
}

#[derive(Serialize)]
struct Version<'a> {
    name: &'a str,
    protocol: i32,
}

#[derive(Serialize)]
struct Players {
    max: usize,
    online: usize,
}

#[derive(Serialize)]
struct Text<'a> {
    text: &'a str,
}

pub fn response_json(
    motd: &str,
    online: usize,
    max_players: usize,
) -> Result<String, serde_json::Error> {
    let response = StatusResponse {
        version: Version {
            name: MINECRAFT_VERSION,
            protocol: PROTOCOL_VERSION,
        },
        players: Players {
            max: max_players,
            online,
        },
        description: Text { text: motd },
        enforces_secure_chat: false,
    };
    serde_json::to_string(&response)
}
