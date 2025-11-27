use crate::err::Result;
use axum::extract::State;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Bytes;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::global::CACHE_ITEM_MIN_SECONDS;
use axum::Json;
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
}

pub async fn convert_url(
    State(state_arc): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<ConvertURLPayload>,
) -> Result<Json<ConvertURLResponse>> {
    let mut hasher = DefaultHasher::new();
    payload.hash(&mut hasher);
    let hash_key = hasher.finish();
    let mut state = state_arc.write().await;
    if state.cache_data.contains_key(&hash_key) {
        return Ok(Json(ConvertURLResponse {
            url: "success".to_string(),
        }));
    }

    let result_bytes = reqwest::get(payload.url.clone()).await?.bytes().await?;
    state.cache_data.insert(hash_key, result_bytes);
    return Ok(Json(ConvertURLResponse {
        url: "success!!".to_string(),
    }));

    // info!("Hello! {}", CACHE_ITEM_MIN_SECONDS);

    Ok(Json(ConvertURLResponse { url: payload.url }))
}
