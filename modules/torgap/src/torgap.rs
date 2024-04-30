// Ref. https://github.com/BlockchainCommons/torgap-demo/blob/master/StackScript/torgap-demo.sh
// This StackScript describes all the setup and functionality of the torgap-demo.

use axum::{
    Router,
    routing::{get, post},
    body::Bytes, http::StatusCode,
    extract::{State, Json},
    response::IntoResponse
};
use log::{debug, error, info, warn};

use home::cargo_home;
use serde::{Serialize, Deserialize};

pub const API_NAME: &str = "torgap-demo";

#[derive(Serialize, Deserialize)]
struct GenerateDidData {
    minisign_key: String,
    document: String
}
pub async fn make_routes() -> Router {
    // @todo Actually define the routes for this API.
    let verify_route = Router::new().route("/verify", get(verify_handler));
    let generate_key_route =  Router::new().route("/generate_key", get(generate_key_handler));

    let generate_did_document_route = Router::new().route("/sign_did",
        post(generate_did_document_handler));

    let api_routes = Router::new()
                    .merge(verify_route)
                    .merge(generate_key_route)
                    .merge(generate_did_document_route);
    Router::new().nest(format!("/{}", API_NAME).as_str(), api_routes)

}

pub async fn start_server() -> anyhow::Result<()> {

    info!("Starting Blockchain Commons Torgap Demo");

    // require torgap-sig-cli-rust
    // Ref. https://github.com/BlockchainCommons/torgap-sig-cli-rust
    {
        rsign("minisign_key".to_string());

    /*
        // @todo Require opentimestamps-client
        opentimestamp("");

        // @todo Start the did-onion tor server.
        start_tor();
    */
    }

    Ok(())
}

async fn verify_handler() -> impl IntoResponse {
    unimplemented!();

    (
        StatusCode::OK,
        "Verify document",
    )
}

async fn generate_key_handler() -> impl IntoResponse {
    // @todo Use Basic Auth for the password to sign the key.
    /*
    @todo Use torgap-sig-cli-rust to convert minisign secret key to Tor secret key
    git clone https://github.com/BlockchainCommons/torgap-sig-cli-rust.git

    cargo run generate -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD <<< $MINISIGN_SECRET_KEY_PASSWORD
    echo "$0 - minisign secret key generated"
    */

    /*
    Inputs for initialization
    MINISIGN_SECRET_KEY
    MINISIGN_SECRET_KEY_PASSWORD   // "Password used to encrypt/decrypt minisign secret key"
    */
    unimplemented!();

    (
        StatusCode::OK,
        "Verify document",
    )
}

async fn generate_did_document_handler(Json(generate_did_data): Json<GenerateDidData>) -> impl IntoResponse {
    // @fixme Don't transmit the key in cleartext.
    // @todo Generate DID document and expose it on our server.
    // The DID document will be exposed for each user.
    // Should a user be able to expose more than 1 DID?
    // @todo Add a route so that DID documents can be retrieved.

    /*
        @todo Use torgap-sig-cli-rust to convert minisign secret key to Tor secret key
        git clone https://github.com/BlockchainCommons/torgap-sig-cli-rust.git

        cargo run generate -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD <<< $MINISIGN_SECRET_KEY_PASSWORD
        echo "$0 - minisign secret key generated"
    */
    rsign(generate_did_data.document);

    /*

        echo "$0 - exporting keys to Tor format"
        cargo run export-to-onion-keys -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD

        # Create a text object to be signed with MINISIGN_SECRET_KEY
        echo "This message is signed by the controller of the same private key used by $(<$TOR_HOSTNAME)" > ~standup/torgap-demo/public/text.txt

        echo "$0 - Signing our text object with minisign secret key"
        ~standup/torgap-sig-cli-rust/target/debug/rsign sign ~standup/torgap-demo/public/text.txt -s "$MINISIGN_SECRET_KEY" -t $(<$TOR_HOSTNAME) <<< $MINISIGN_SECRET_KEY_PASSWORD

        # Make a timestamp of our signature with OpenTimestamps
    sudo apt-get install -y python3 python3-dev python3-pip python3-setuptools python3-wheel
    pip3 install opentimestamps-client
    rm ~standup/torgap-demo/public/text.txt.minisig.ots
    ots stamp ~standup/torgap-demo/public/text.txt.minisig

        */
    unimplemented!();

    (
        StatusCode::OK,
        "Verify document".to_string(),
    )
}

fn export_to_onion_keys() {
    // @todo
    // cargo run export-to-onion-keys -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD
    unimplemented!();
}

fn sign_message_with_minisign_secret() {
    /*
    Create a text object to be signed with MINISIGN_SECRET_KEY
    echo "This message is signed by the controller of the same private key used by $(<$TOR_HOSTNAME)" > ~standup/torgap-demo/public/text.txt

    echo "$0 - Signing our text object with minisign secret key"
    ~standup/torgap-sig-cli-rust/target/debug/rsign sign ~standup/torgap-demo/public/text.txt -s "$MINISIGN_SECRET_KEY" -t $(<$TOR_HOSTNAME) <<< $MINISIGN_SECRET_KEY_PASSWORD
    */
    unimplemented!()
}

fn get_onion_address() {
    /*
    set our onion address in our index.html
    cargo run verify text.txt --onion-address $(<$TOR_HOSTNAME) /g" ~standup/torgap-demo/public/index.html

    Make a timestamp of our signature with OpenTimestamps
    sudo apt-get install -y python3 python3-dev python3-pip python3-setuptools python3-wheel
    pip3 install opentimestamps-client
    rm ~standup/torgap-demo/public/text.txt.minisig.ots
    ots stamp ~standup/torgap-demo/public/text.txt.minisig
    */
    unimplemented!();
}

use std::process::Command;
const TORGAP_CLI_COMMAND: &str = "rsign";

fn rsign(minisign_key: String) {
    // Ref. https://github.com/BlockchainCommons/torgap-demo/blob/ed73f1cab8f87093823c452450b9d1fe540ec9f5/StackScript/torgap-demo.sh#L202

    // @todo Use torgap-sig-cli-rust
    // @todo Use CARGO_PATH or add CARGO_PATH to the dockerfile.
    use std::path::PathBuf;
    let path: PathBuf = [cargo_home().unwrap(), "bin".into(), TORGAP_CLI_COMMAND.into()].iter().collect();
    dbg!(&path);
    let mut rsign = Command::new(&path);
    assert!(
        !rsign.get_program().is_empty(),
        "{TORGAP_CLI_COMMAND} not found in path"
    );
    let secret_key_path = "@todo";
    if !minisign_key.is_empty() {
        let output = rsign.args(["generate", "-s", secret_key_path ] )
                .output()
                .expect("rsign failed");
        dbg!(output);
    }
}

#[test]
fn test_rsign() {
    rsign("0xFFFF");
    unimplemented!();
}

const OPENTIMESTAMP_CLIENT: &str = "ots";
fn opentimestamp(command: &str) {
    let ots = Command::new(OPENTIMESTAMP_CLIENT);
    assert!(
        !ots.get_program().is_empty(),
        "{OPENTIMESTAMP_CLIENT} not found in path"
    );
    if !command.is_empty() {
        unimplemented!()
    }
}

fn start_tor() {
    /*
    TOR_SECRET_KEY
    TOR_PUBLIC_KEY
    TOR_HOSTNAME
     */
    unimplemented!();
}
