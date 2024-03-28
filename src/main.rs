mod api;
mod logging;
mod modules;
mod server;

use log::error;
use logging::setup_log;
use nu_ansi_term::Color::Red;
use server::start_server;

#[tokio::main]
async fn main() {
    setup_log();

    let schema_name = "depo";

    // @todo Each module should have a start function that the server calls.
    if let Err(e) = start_server(schema_name, 5332).await {
        error!(
            "{}",
            Red.paint("Could not start server. Is the database running?")
                .to_string()
        );
        error!("{}", Red.paint(format!("{}", e)).to_string());
    };
}
