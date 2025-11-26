use axum::Json;
use serde::{Deserialize, Serialize};

// TODO check whether we only need Serialize or Deserialize
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertURLPayload {
    pub url: String,
    pub extension_input: String,
    pub extension_output: String,
}

pub async fn convert_url(Json(payload): Json<ConvertURLPayload>) -> String {
    payload.url
}
