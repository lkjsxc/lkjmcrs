use crate::protocol::codec;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct DecodedRegistry {
    pub id: String,
    pub entries: Vec<String>,
}

#[derive(Debug)]
pub struct DecodedTagGroup {
    pub registry: String,
    pub tags: Vec<DecodedTag>,
}

#[derive(Debug)]
pub struct DecodedTag {
    pub name: String,
    pub entries: Vec<i32>,
}

pub fn decode_registry_data(data: Vec<u8>) -> Result<DecodedRegistry, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let id = codec::read_string(&mut cursor)?;
    let count = codec::read_var_i32(&mut cursor)?;
    let mut entries = Vec::new();
    for _ in 0..count {
        entries.push(codec::read_string(&mut cursor)?);
        if !codec::read_bool(&mut cursor)? {
            continue;
        }
        skip_anonymous_compound(&mut cursor);
    }
    assert_finished(&cursor, "registry data trailing bytes")?;
    Ok(DecodedRegistry { id, entries })
}

pub fn decode_tags(data: Vec<u8>) -> Result<Vec<DecodedTagGroup>, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let count = codec::read_var_i32(&mut cursor)?;
    let mut groups = Vec::new();
    for _ in 0..count {
        let registry = codec::read_string(&mut cursor)?;
        let tag_count = codec::read_var_i32(&mut cursor)?;
        let mut tags = Vec::new();
        for _ in 0..tag_count {
            let name = codec::read_string(&mut cursor)?;
            let entry_count = codec::read_var_i32(&mut cursor)?;
            let mut entries = Vec::new();
            for _ in 0..entry_count {
                entries.push(codec::read_var_i32(&mut cursor)?);
            }
            tags.push(DecodedTag { name, entries });
        }
        groups.push(DecodedTagGroup { registry, tags });
    }
    assert_finished(&cursor, "tags trailing bytes")?;
    Ok(groups)
}

fn assert_finished(
    cursor: &Cursor<Vec<u8>>,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            phase,
        )));
    }
    Ok(())
}

fn skip_anonymous_compound(cursor: &mut Cursor<Vec<u8>>) {
    assert_eq!(read_u8(cursor), 10);
    skip_compound_payload(cursor);
}

fn skip_compound_payload(cursor: &mut Cursor<Vec<u8>>) {
    loop {
        let tag = read_u8(cursor);
        if tag == 0 {
            return;
        }
        skip_nbt_string(cursor);
        skip_payload(cursor, tag);
    }
}

fn skip_payload(cursor: &mut Cursor<Vec<u8>>, tag: u8) {
    match tag {
        1 => skip(cursor, 1),
        3 | 5 => skip(cursor, 4),
        6 => skip(cursor, 8),
        8 => skip_nbt_string(cursor),
        10 => skip_compound_payload(cursor),
        other => panic!("unsupported tag {other}"),
    }
}

fn skip_nbt_string(cursor: &mut Cursor<Vec<u8>>) {
    let len = codec::read_u16(cursor).unwrap() as usize;
    skip(cursor, len);
}

fn skip(cursor: &mut Cursor<Vec<u8>>, len: usize) {
    let mut ignored = vec![0; len];
    cursor.read_exact(&mut ignored).unwrap();
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> u8 {
    let mut byte = [0];
    cursor.read_exact(&mut byte).unwrap();
    byte[0]
}
