use crate::protocol::codec;

const ROOT: u8 = 0x00;
const LITERAL: u8 = 0x01;
const ARGUMENT: u8 = 0x02;
const EXECUTABLE: u8 = 0x04;
const STRING_PARSER: i32 = 5;
const SINGLE_WORD: i32 = 0;
const GREEDY_PHRASE: i32 = 2;

struct Node<'a> {
    flags: u8,
    children: &'a [i32],
    name: Option<&'a str>,
    parser: Option<i32>,
    string_kind: Option<i32>,
}

pub fn encode_declare_commands() -> Vec<u8> {
    let nodes = [
        node(ROOT, &[1, 2, 3, 5, 7, 8, 10, 12, 13, 15, 18, 21], None),
        node(LITERAL | EXECUTABLE, &[], Some("help")),
        node(LITERAL | EXECUTABLE, &[], Some("spawn")),
        node(LITERAL | EXECUTABLE, &[4], Some("sethome")),
        string_arg("name", SINGLE_WORD, &[]),
        node(LITERAL | EXECUTABLE, &[6], Some("home")),
        string_arg("name", SINGLE_WORD, &[]),
        node(LITERAL | EXECUTABLE, &[], Some("homes")),
        node(LITERAL, &[9], Some("setwarp")),
        string_arg("name", SINGLE_WORD, &[]),
        node(LITERAL, &[11], Some("warp")),
        string_arg("name", SINGLE_WORD, &[]),
        node(LITERAL | EXECUTABLE, &[], Some("warps")),
        node(LITERAL, &[14], Some("say")),
        string_arg("message", GREEDY_PHRASE, &[]),
        node(LITERAL, &[16], Some("gamemode")),
        string_arg("mode", SINGLE_WORD, &[17]),
        string_arg("player", SINGLE_WORD, &[]),
        node(LITERAL, &[19], Some("kick")),
        string_arg("player", SINGLE_WORD, &[20]),
        string_arg("reason", GREEDY_PHRASE, &[]),
        node(LITERAL, &[22], Some("damage")),
        string_arg("player", SINGLE_WORD, &[23]),
        string_arg("amount", SINGLE_WORD, &[]),
    ];
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, nodes.len() as i32);
    for node in nodes {
        write_node(&mut out, node);
    }
    codec::write_var_i32(&mut out, 0);
    out
}

fn node<'a>(flags: u8, children: &'a [i32], name: Option<&'a str>) -> Node<'a> {
    Node {
        flags,
        children,
        name,
        parser: None,
        string_kind: None,
    }
}

fn string_arg<'a>(name: &'a str, kind: i32, children: &'a [i32]) -> Node<'a> {
    Node {
        flags: ARGUMENT | EXECUTABLE,
        children,
        name: Some(name),
        parser: Some(STRING_PARSER),
        string_kind: Some(kind),
    }
}

fn write_node(out: &mut Vec<u8>, node: Node<'_>) {
    codec::write_u8(out, node.flags);
    codec::write_var_i32(out, node.children.len() as i32);
    for child in node.children {
        codec::write_var_i32(out, *child);
    }
    if let Some(name) = node.name {
        codec::write_string(out, name);
    }
    if let Some(parser) = node.parser {
        codec::write_var_i32(out, parser);
        codec::write_var_i32(out, node.string_kind.unwrap_or(SINGLE_WORD));
    }
}

#[cfg(test)]
mod tests {
    use super::encode_declare_commands;

    #[test]
    fn command_tree_declares_nodes() {
        let payload = encode_declare_commands();
        assert_eq!(payload[0], 24);
        assert!(String::from_utf8_lossy(&payload).contains("sethome"));
        assert!(String::from_utf8_lossy(&payload).contains("gamemode"));
        assert!(String::from_utf8_lossy(&payload).contains("damage"));
    }
}
