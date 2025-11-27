use std::collections::HashMap;
use bytes::Bytes;
// use std::io::Bytes;

type ItemHash = u64;
type TimestampSecond = i64;

pub struct AppState {
    /// When was an item stored in cache
    pub cache_time: HashMap<ItemHash, TimestampSecond>,
    /// The data that was cached
    pub cache_data: HashMap<ItemHash, Bytes>,

    pub cache_total_bytes: usize,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            cache_time: HashMap::new(),
            cache_data: HashMap::new(),
            cache_total_bytes: 0,
        }
    }
}
