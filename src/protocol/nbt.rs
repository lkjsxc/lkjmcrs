use crate::protocol::codec;

#[derive(Debug, Clone)]
pub struct Compound(Vec<(String, Tag)>);

#[derive(Debug, Clone)]
pub enum Tag {
    Byte(i8),
    Int(i32),
    Float(f32),
    Double(f64),
    String(String),
    Compound(Compound),
}

pub fn write_anonymous_compound(out: &mut Vec<u8>, compound: &Compound) {
    codec::write_u8(out, 10);
    write_compound_payload(out, compound);
}

pub fn compound(values: Vec<(&str, Tag)>) -> Compound {
    Compound(
        values
            .into_iter()
            .map(|(name, tag)| (name.to_string(), tag))
            .collect(),
    )
}

pub fn string(value: &str) -> Tag {
    Tag::String(value.to_string())
}

fn write_compound_payload(out: &mut Vec<u8>, compound: &Compound) {
    for (name, tag) in &compound.0 {
        codec::write_u8(out, tag.id());
        write_nbt_string(out, name);
        tag.write_payload(out);
    }
    codec::write_u8(out, 0);
}

fn write_nbt_string(out: &mut Vec<u8>, value: &str) {
    codec::write_u16(out, value.len() as u16);
    out.extend_from_slice(value.as_bytes());
}

impl Tag {
    const fn id(&self) -> u8 {
        match self {
            Self::Byte(_) => 1,
            Self::Int(_) => 3,
            Self::Float(_) => 5,
            Self::Double(_) => 6,
            Self::String(_) => 8,
            Self::Compound(_) => 10,
        }
    }

    fn write_payload(&self, out: &mut Vec<u8>) {
        match self {
            Self::Byte(value) => codec::write_i8(out, *value),
            Self::Int(value) => codec::write_i32(out, *value),
            Self::Float(value) => codec::write_f32(out, *value),
            Self::Double(value) => codec::write_f64(out, *value),
            Self::String(value) => write_nbt_string(out, value),
            Self::Compound(value) => write_compound_payload(out, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tag, compound, string, write_anonymous_compound};

    #[test]
    fn writes_anonymous_compound_without_root_name() {
        let mut out = Vec::new();
        write_anonymous_compound(&mut out, &compound(vec![("key", string("value"))]));
        assert_eq!(out[0], 10);
        assert_eq!(out[1], 8);
        assert_eq!(&out[2..7], b"\0\x03key");
        assert_eq!(out.last(), Some(&0));
    }

    #[test]
    fn writes_numeric_tags() {
        let mut out = Vec::new();
        write_anonymous_compound(
            &mut out,
            &compound(vec![("byte", Tag::Byte(1)), ("int", Tag::Int(2))]),
        );
        assert!(out.contains(&1));
        assert!(out.contains(&3));
    }
}
