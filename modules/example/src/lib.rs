use axum::{
    Router,
    routing::get,
    http::StatusCode,
    extract::Json,
    response::IntoResponse
};

use log::{debug, error, info};

pub const API_NAME: &str = "example";

pub async fn make_routes() -> Router {
    let api_routes = Router::new().route("/test", get(example_handler));
    Router::new().nest(format!("/{}", API_NAME).as_str(), api_routes)
}

pub async fn start_server() -> anyhow::Result<()> {
    debug!("example::start_server");
    Ok(())
}

async fn example_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        "Example is running".to_string()
    )
}