use env_logger::Builder;
use log::LevelFilter;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_log() {
    INIT.call_once(|| {
        Builder::new()
            .filter(None, LevelFilter::Debug) // @todo Make this an environment variable.
            .init();
        // Builder::from_env(
        //     Env::default()
        //         .default_filter_or("depo=info")
        //     )
        //     .init();
        // println!("LOGGING INITIALIZED")
    });
}
