// Ref. https://github.com/BlockchainCommons/torgap-demo/blob/master/StackScript/torgap-demo.sh
use log::info;
use nu_ansi_term::Color::Green;
use warp::{
    filters::BoxedFilter,
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

pub const API_NAME: &str = "torgap";

pub async fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let verify_route = warp::path::end().and_then(verify_handler);

    let generate_key_route = warp::path::end().and_then(generate_key_handler);

    let generate_did_document_route = warp::path::end().and_then(generate_did_document_handler);

    let routes = verify_route
        .or(generate_key_route)
        .or(generate_did_document_route);

    routes.boxed()
}

pub async fn start_server() -> anyhow::Result<()> {
    // require torgap-sig-cli-rust
    // Ref. https://github.com/BlockchainCommons/torgap-sig-cli-rust
    /*
    {
        rsign("");

        // @todo Require opentimestamps-client
        opentimestamp("");

        // @todo Start the did-onion tor server.
        start_tor();
    }
    */

    info!(
        "{}",
        Green.paint(format!("Starting Blockchain Commons Torgap Demo"))
    );

    Ok(())
}

async fn verify_handler() -> Result<Box<dyn Reply>, Rejection> {
    unimplemented!();
    Ok(Box::new(reply::with_status(
        "Verify document",
        StatusCode::OK,
    )))
}

async fn generate_key_handler() -> Result<Box<dyn Reply>, Rejection> {
    // @todo Use Basic Auth for the password to sign the key.
    /*
    @todo Use torgap-sig-cli-rust to convert minisign secret key to Tor secret key
    git clone https://github.com/BlockchainCommons/torgap-sig-cli-rust.git

    cargo run generate -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD <<< $MINISIGN_SECRET_KEY_PASSWORD
    echo "$0 - minisign secret key generated"
    */
    unimplemented!();
    Ok(Box::new(reply::with_status(
        "Verify document",
        StatusCode::OK,
    )))
}

async fn generate_did_document_handler() -> Result<Box<dyn Reply>, Rejection> {
    // @todo Use Basic Auth for the password to sign the key.
    // @todo Generate DID document and expose it on our server.
    // The DID document will be exposed for each user.
    // Should a user be able to expose more than 1 DID?
    // @todo Add a route so that DID documents can be retrieved.

    /*
        @todo Use torgap-sig-cli-rust to convert minisign secret key to Tor secret key
        git clone https://github.com/BlockchainCommons/torgap-sig-cli-rust.git

        cargo run generate -s $MINISIGN_SECRET_KEY <<< $MINISIGN_SECRET_KEY_PASSWORD <<< $MINISIGN_SECRET_KEY_PASSWORD
        echo "$0 - minisign secret key generated"

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
    Ok(Box::new(reply::with_status(
        "Verify document",
        StatusCode::OK,
    )))
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

fn rsign(command: &str) {
    let rsign = Command::new(TORGAP_CLI_COMMAND);
    assert!(
        !rsign.get_program().is_empty(),
        "{TORGAP_CLI_COMMAND} not found in path"
    );
    if !command.is_empty() {
        unimplemented!()
    }
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
    unimplemented!();
}
