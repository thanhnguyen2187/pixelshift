use crate::err::{Error, Result};
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use image::{AnimationDecoder, ImageFormat, ImageReader};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::global::MAX_DOWNLOAD_SIZE_BYTES;
use axum::response::IntoResponse;
use bytes::Bytes;
use image::codecs::gif;
use image::codecs::gif::GifEncoder;
use image::codecs::webp::WebPDecoder;
use tracing::{debug, warn};

pub async fn convert_url_v2(
    State(state_arc): State<Arc<RwLock<AppState>>>,
    Path((extension_output, url)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let mut hasher = DefaultHasher::new();
    (&extension_output, &url).hash(&mut hasher);
    let hash_key = hasher.finish();

    let state_read = state_arc.read().await;
    let is_in_cache = state_read.cache_data.contains(&hash_key);
    // Do early drop to avoid later deadlock
    drop(state_read);

    let bytes_result;
    if !is_in_cache {
        debug!("Cache wasn't hit for URL {}", &url);

        let response = reqwest::get(&url).await?;
        if let Some(file_size_bytes) = response.content_length() {
            if file_size_bytes > *MAX_DOWNLOAD_SIZE_BYTES {
                warn!("File from URL {} is too large!", &url);
                return Err(Error::DownloadLargeFile { url });
            }
        }
        let bytes_input = response.bytes().await?;
        debug!("Downloaded URL {}", &url);

        let reader = ImageReader::new(Cursor::new(bytes_input.clone())).with_guessed_format()?;
        match reader.format() {
            Some(ImageFormat::WebP) => {}
            Some(ImageFormat::Gif) => {
                return Ok((
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "image/gif")],
                    bytes_input,
                ));
            }
            _ => {
                return Err(Error::URLContent {});
            }
        }

        let frames = WebPDecoder::new(reader.into_inner())?
            .into_frames()
            .collect_frames()?;
        debug!("Turned URL {} into {} frame(s)", &url, frames.len());

        let mut bytes_output: Vec<u8> = Vec::new();
        let cursor_output = Cursor::new(&mut bytes_output);
        // Set higher speed to make the process faster
        let mut encoder = GifEncoder::new_with_speed(cursor_output, 15);
        encoder.set_repeat(gif::Repeat::Infinite)?;
        encoder.encode_frames(frames)?;
        drop(encoder);
        debug!("Converted URL {} into GIF", &url);

        let mut state_write = state_arc.write().await;
        bytes_result = Bytes::from(bytes_output);
        state_write.cache_data.put(hash_key, bytes_result.clone());

        debug!("Stored output of URL {} into cache", &url);
    } else {
        let mut state_write = state_arc.write().await;
        bytes_result = state_write
            .cache_data
            .get(&hash_key)
            .expect("unreachable code")
            .clone();
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/gif")],
        bytes_result,
    ))
}
