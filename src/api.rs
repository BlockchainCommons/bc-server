pub const MAX_DATA_SIZE: u32 = 1000;
pub const CONTINUATION_EXPIRY_SECONDS: u32 = 60 * 60 * 24;

// API
pub mod types {
    /*
    pub type Routes =
        impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone;
    */
}

#[derive(Debug)]
pub struct InvalidBody;
impl warp::reject::Reject for InvalidBody {}

#[derive(Debug)]
pub struct AnyhowError(anyhow::Error);
impl warp::reject::Reject for AnyhowError {}

use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
};
