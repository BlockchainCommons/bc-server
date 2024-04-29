use log::info;
use nu_ansi_term::Color::Green;
use axum::{routing::get, Router};

#[cfg(feature = "depo-module")]
use depo_module;

pub async fn start_server(schema_name: &str, port: u16) -> anyhow::Result<()> {

    let mut app = Router::new();

    // @todo Loop through all modules and get routes.
    let module_routes = make_routes().await;
    app = app.merge(module_routes);

    {
        // call module.start_server for all modules.
        #[cfg(feature = "depo-module")]
        depo_module::start_server().await;

        #[cfg(feature = "torgap")]
        torgap::start_server().await;
    }

    let host = "0.0.0.0"; // 127.0.0.1 won't work inside docker.
    let addr = format!("{}:{}", host, port);
    let addr = addr.parse::<std::net::SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn make_routes() -> Router {

    let mut app = status_router();

    #[cfg(feature = "depo-module")]
    {
        let depo_routes = depo_module::make_routes().await;
        app = app.nest("/api", depo_routes);
    }

    #[cfg(feature = "torgap")]
    {
        let torgap_routes = torgap::make_routes().await;
        app = app.nest("/api", torgap_routes);
    }

    // @todo A module returns an axum::Router
    // Add new router here using a conditional block as above.

    dbg!(&app);
    app


}

// Ref. https://github.com/seanmonstar/warp/blob/master/examples/dyn_reply.rs
fn status_router() -> Router {
    Router::new().route("/status", get(|| async { "Server is running".to_string() }))
}

// @fixme Tests
/*
#[tokio::test]
async fn test_status() {
    let filter = status_filter();
    let result = warp::test::request()
        .path("/api/status")
        .reply(&filter)
        .await;
    assert_eq!(result.status(), 200, "{}", result.status());
    let app = Router::new().route("")
}

*/