use crate::err::Result;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use image::{AnimationDecoder, ImageFormat, ImageReader};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::global::CACHE_ITEM_MIN_SECONDS;
use axum::Json;
use axum::response::IntoResponse;
use bytes::Bytes;
use image::codecs::gif;
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::codecs::webp::WebPDecoder;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::{debug, info};

// TODO check whether we only need Serialize or Deserialize
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertURLPayload {
    pub url: String,
    pub extension_input: String,
    pub extension_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertURLResponse {
    pub url: String,
    pub cache_hit: bool,
}

pub async fn convert_url(
    State(state_arc): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<ConvertURLPayload>,
) -> Result<Json<ConvertURLResponse>> {
    let mut hasher = DefaultHasher::new();
    payload.hash(&mut hasher);
    let hash_key = hasher.finish();
    let state_read = state_arc.read().await;
    let mut cache_hit = true;
    if !state_read.cache_data.contains_key(&hash_key) {
        debug!("Cache wasn't hit for URL {}", payload.url);
        let bytes_input = reqwest::get(payload.url.clone()).await?.bytes().await?;
        debug!("Downloaded URL {}", payload.url);
        let reader = ImageReader::new(Cursor::new(bytes_input)).with_guessed_format()?;
        let frames = WebPDecoder::new(reader.into_inner())?
            .into_frames()
            .collect_frames()?;
        debug!("Turned URL {} into {} frame(s)", payload.url, frames.len());

        let mut bytes_output: Vec<u8> = Vec::new();
        let cursor_output = Cursor::new(&mut bytes_output);
        let mut encoder = GifEncoder::new(cursor_output);
        encoder.set_repeat(gif::Repeat::Infinite)?;
        encoder.encode_frames(frames)?;
        drop(encoder);
        debug!("Converted URL {} into GIF", payload.url);

        let mut state_write = state_arc.write().await;
        state_write
            .cache_data
            .insert(hash_key, Bytes::from(bytes_output));
        debug!("Stored output of URL {} into cache", payload.url);
        cache_hit = false;
    }

    Ok(Json(ConvertURLResponse {
        url: format!("/api/v1/output/{hash_key}"),
        cache_hit,
    }))
}

pub async fn output(
    State(state_arc): State<Arc<RwLock<AppState>>>,
    Path(hash_id): Path<u64>,
) -> impl IntoResponse {
    let state = state_arc.read().await;
    let bytes = state.cache_data.get(&hash_id).cloned();

    if let Some(bytes) = bytes {
        return (StatusCode::OK, bytes);
    }

    (StatusCode::NOT_FOUND, vec![].into())
}
