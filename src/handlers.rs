use crate::err::Result;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::global::CACHE_ITEM_MIN_SECONDS;
use axum::Json;
use axum::response::IntoResponse;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::info;

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
    let mut state = state_arc.write().await;
    let mut cache_hit = true;
    if !state.cache_data.contains_key(&hash_key) {
        let result_bytes = reqwest::get(payload.url.clone()).await?.bytes().await?;
        state.cache_total_bytes += result_bytes.len();
        state.cache_data.insert(hash_key, result_bytes);
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

    (StatusCode::NOT_FOUND, Bytes::new())
}
