use crate::protocol::chunk;
use crate::session::chunk_wire::WireChunk;
use crate::world::{ChunkSnapshot, GeneratedChunkKey};
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};

static FLAT_BODY: OnceLock<Vec<u8>> = OnceLock::new();
static GENERATED_PAYLOADS: OnceLock<Mutex<GeneratedPayloadCache>> = OnceLock::new();
const GENERATED_CACHE_CAP: usize = 8192;

#[derive(Debug, Default)]
pub struct ChunkPayloadCache {
    stats: ChunkPayloadCacheStats,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPayloadCacheStats {
    pub flat_hits: usize,
    pub flat_misses: usize,
    pub generated_hits: usize,
    pub generated_misses: usize,
    pub generated_evictions: usize,
    pub override_bypasses: usize,
}

impl ChunkPayloadCache {
    pub fn encode(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if snapshot.is_shared_flat_base() {
            return self.encode_flat(snapshot);
        }
        if let Some(key) = snapshot.generated_cache_key() {
            return self.encode_generated(snapshot, key);
        }
        self.stats.override_bypasses += 1;
        chunk::encode_level_chunk_with_light(&WireChunk(snapshot))
    }

    pub fn stats(&self) -> ChunkPayloadCacheStats {
        self.stats
    }

    fn encode_flat(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if FLAT_BODY.get().is_some() {
            self.stats.flat_hits += 1;
        } else {
            self.stats.flat_misses += 1;
        }
        let body = FLAT_BODY
            .get_or_init(|| chunk::encode_level_chunk_body_with_light(&WireChunk(snapshot)));
        with_position(snapshot, body)
    }

    fn encode_generated(&mut self, snapshot: &ChunkSnapshot, key: GeneratedChunkKey) -> Vec<u8> {
        let cache = GENERATED_PAYLOADS
            .get_or_init(|| Mutex::new(GeneratedPayloadCache::new(GENERATED_CACHE_CAP)));
        let mut cache = cache.lock().expect("generated payload cache poisoned");
        match cache.get_or_insert_with(key, || {
            chunk::encode_level_chunk_with_light(&WireChunk(snapshot))
        }) {
            GeneratedCacheResult::Hit(payload) => {
                self.stats.generated_hits += 1;
                payload
            }
            GeneratedCacheResult::Miss { payload, evicted } => {
                self.stats.generated_misses += 1;
                self.stats.generated_evictions += usize::from(evicted);
                payload
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct GeneratedPayloadCache {
    cap: usize,
    order: VecDeque<GeneratedChunkKey>,
    payloads: HashMap<GeneratedChunkKey, Vec<u8>>,
}

enum GeneratedCacheResult {
    Hit(Vec<u8>),
    Miss { payload: Vec<u8>, evicted: bool },
}

impl GeneratedPayloadCache {
    pub(super) fn new(cap: usize) -> Self {
        Self {
            cap,
            order: VecDeque::new(),
            payloads: HashMap::new(),
        }
    }

    fn get_or_insert_with(
        &mut self,
        key: GeneratedChunkKey,
        encode: impl FnOnce() -> Vec<u8>,
    ) -> GeneratedCacheResult {
        if let Some(payload) = self.payloads.get(&key) {
            return GeneratedCacheResult::Hit(payload.clone());
        }
        let payload = encode();
        let evicted = self.insert(key, payload.clone());
        GeneratedCacheResult::Miss { payload, evicted }
    }

    pub(super) fn insert(&mut self, key: GeneratedChunkKey, payload: Vec<u8>) -> bool {
        if self.cap == 0 {
            return false;
        }
        let mut evicted = false;
        while self.payloads.len() >= self.cap {
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            evicted |= self.payloads.remove(&oldest).is_some();
        }
        self.order.push_back(key);
        self.payloads.insert(key, payload);
        evicted
    }

    #[cfg(test)]
    pub(super) fn contains(&self, key: GeneratedChunkKey) -> bool {
        self.payloads.contains_key(&key)
    }
}

fn with_position(snapshot: &ChunkSnapshot, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(&snapshot.pos.x.to_be_bytes());
    out.extend_from_slice(&snapshot.pos.z.to_be_bytes());
    out.extend_from_slice(body);
    out
}
