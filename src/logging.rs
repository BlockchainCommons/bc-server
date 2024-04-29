// use env_logger::Builder;w
use log::{debug, LevelFilter};
use std::sync::Once;

use pretty_env_logger;

static INIT: Once = Once::new();

pub fn setup_log() {
    INIT.call_once(|| {
        /*
               Builder::new()
                   .filter(None, LevelFilter::Debug)
                   .init();
        */
        // Builder::from_default_env().init();
        pretty_env_logger::init();
        debug!("LOGGING INITIALIZED");
    });
}
