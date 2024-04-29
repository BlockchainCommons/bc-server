use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use anyhow;

pub const MAX_DATA_SIZE: u32 = 1000;
pub const CONTINUATION_EXPIRY_SECONDS: u32 = 60 * 60 * 24;

// API
#[derive(Debug)]
pub struct InvalidBodyError(anyhow::Error);
impl IntoResponse for InvalidBodyError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            format!("The request body was not formatted corectly: {}", self.0),
        ).into_response()
    }
}

/*
use std::error::Error;
impl Error for InvalidBodyError {}

use std::fmt;
impl fmt::Display for InvalidBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
*/

impl<E> From<E> for InvalidBodyError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
