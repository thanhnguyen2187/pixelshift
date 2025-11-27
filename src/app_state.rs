use crate::global::CACHE_MAX_SIZE;
use bytes::Bytes;
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct AppState {
    pub cache_data: LruCache<u64, Bytes>,
}

impl AppState {
    pub fn new() -> Self {
        let size = NonZeroUsize::new(*CACHE_MAX_SIZE).expect("unreachable code");
        AppState {
            cache_data: LruCache::new(size),
        }
    }
}
