// @todo Pass a logger to the module.

use anyhow::Result;

use axum::{
    Router,
    routing::{get, post},
    body::Bytes, http::StatusCode,
    extract::State,
    response::IntoResponse
};
use log::{debug, error, info, warn};

// use nu_ansi_term::Color::{Green, Red, Yellow};

const SCHEMA_NAME: &str = "depo";
pub const API_NAME: &str = "depo";

use depo::{create_db, reset_db, server_pool, Depo};


pub async fn make_routes() -> Router {
    // @todo Each module will be have it's own path.
    // e.g. /api/depo/ for depo
    // e.g. /api/timestamp/ for timestamp

    dbg!("make_routes");
    create_db(&server_pool(), SCHEMA_NAME).await;
    // @fixme Why do we need this new_db call?
    let depo = Depo::new_db(SCHEMA_NAME).await.unwrap();

    let api_routes = Router::new()
                        .route("/", get(key_handler).post(operation_handler))
                        .route("/reset-db", post(|State(depo): State<Depo>| async {
                            reset_db_handler(SCHEMA_NAME.to_owned().clone()).await
                        }))
                        .with_state(depo.clone());
    Router::new().nest(format!("/{}", API_NAME).as_str(), api_routes)
}

pub async fn start_server() -> anyhow::Result<()> {
    let depo = Depo::new_db(SCHEMA_NAME).await?;

    info!("Starting Blockchain Commons Depository");
    debug!("{}", format!("Public key: {}", depo.public_key_string()));

    Ok(())
}
async fn key_handler(State(depo): State<Depo>) -> impl IntoResponse {
   debug!("key_handler");
   (
        StatusCode::OK,
        depo.public_key_string().to_string(),
    )
}

async fn operation_handler(State(depo): State<Depo>,
    /* Verify that this is how an Extractor works to get the body of the request. */
    body: Bytes) -> impl IntoResponse
{
    debug!("operation_handler");
    match String::from_utf8(body.to_vec()) {
        Ok(b) => {
            let result = depo.handle_request_string(b).await;
            Ok((StatusCode::OK, result))
        },
        Err(e) => Err((StatusCode::BAD_REQUEST,
                        format!("{}", e)))
    }
}

// @todo
async fn reset_db_handler(schema_name: String) -> impl IntoResponse {
    debug!("rest_db_handler");
    match reset_db(&schema_name).await {
        Ok(_) => (StatusCode::OK, "Database reset successfully. A new private key has been assigned. Server must be restarted.".to_string()),
        Err(e) => {
            let error_message = format!("Failed to reset database: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message)
        },
    }
}
