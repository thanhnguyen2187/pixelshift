use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use snafu::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(display("I/O error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },

    #[snafu(display("Making request error: {source}"))]
    MakingRequest { source: reqwest::Error },

    #[snafu(display("Image error: {source}"))]
    ImageError { source: image::ImageError },

    #[snafu(display("Input URL file is too large: {url}"))]
    DownloadLargeFile { url: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DownloadLargeFile { url: _ } => {
                (StatusCode::BAD_REQUEST, format!("Bad input: {}", self)).into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self),
            )
                .into_response(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io { source }
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::MakingRequest { source }
    }
}

impl From<image::ImageError> for Error {
    fn from(source: image::ImageError) -> Self {
        Error::ImageError { source }
    }
}
