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

use bc_server_api::InvalidBodyError;
use depo::{create_db, reset_db, server_pool, Depo};


pub async fn make_routes() -> Router {
    // @todo Each module will be have it's own path.
    // e.g. /api/depo/ for depo
    // e.g. /api/timestamp/ for timestamp

    dbg!("make_routes");
    create_db(&server_pool(), SCHEMA_NAME).await;
    // @fixme Why do we need this new_db call?
    let depo = Depo::new_db(SCHEMA_NAME).await.unwrap();
    Router::new()
                .route("/", get(|_: State<Depo>| async {}))
                /*
                // @fixme
                .route("/", get(|State(depo): State<Depo>| async {  key_handler(depo) }))
                .route("/", post(|State(depo): State<Depo>, body: Bytes| async {
                    // @todo Get request body in bytes.
                    operation_handler(depo, body)
                }))
                .route("reset-db", post(|State(depo): State<Depo>| async {
                    reset_db_handler(SCHEMA_NAME.to_owned().clone())
                }))
 */                .with_state(depo.clone())
}

pub async fn start_server() -> anyhow::Result<()> {
    let depo = Depo::new_db(SCHEMA_NAME).await?;

    info!("Starting Blockchain Commons Depository");
    debug!("{}", format!("Public key: {}", depo.public_key_string()));

    Ok(())
}
async fn key_handler(depo: Depo) -> impl IntoResponse {
   debug!("key_handler");
   (
        StatusCode::OK,
        depo.public_key_string().to_string(),
    )
}

async fn operation_handler(depo: Depo, body: Bytes) -> Result<impl IntoResponse, InvalidBodyError> {
    debug!("operation_handler");
    let body_string = std::str::from_utf8(&body)
        .map_err( |e| {
            // @fixme Return an error response. An unhandled error will crash server.
            error!("{}", e);
            e
        } )?
        .to_string();
    let a = depo.handle_request_string(body_string).await;

    Ok((StatusCode::OK, a))
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
