use log::info;
use nu_ansi_term::Color::Green;
use warp::{
    filters::BoxedFilter,
    reject::Rejection,
    reply::{self, Reply},
    Filter,
};

use crate::modules::depo;
use crate::modules::torgap;

pub async fn start_server(schema_name: &str, port: u16) -> anyhow::Result<()> {
    // @todo Loop through all modules and get routes.
    let routes = make_routes().await;

    let host = "0.0.0.0"; // 127.0.0.1 won't work inside docker.
    let addr = format!("{}:{}", host, port);
    let socket_addr = addr.parse::<std::net::SocketAddr>()?;

    {
        depo::start_server().await;
        torgap::start_server().await;
    }

    warp::serve(routes).run(socket_addr).await;

    Ok(())
}

async fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let result = {
        let api = warp::path("api");
        let status = status_filter();
        let depo_route = api.and(warp::path(depo::API_NAME));
        let depo_route = depo_route.and(depo::make_routes().await);
        let torgap_route = api.and(warp::path(torgap::API_NAME));
        let torgap_route = torgap_route.and(torgap::make_routes().await);
        status.or(depo_route).or(torgap_route)
    };

    result.boxed()
}

fn status_filter() -> BoxedFilter<(impl Reply,)> {
    let api = warp::path("api");

    api.and(warp::path!("status"))
        .and_then(status_handler)
        .boxed()
}

// Ref. https://github.com/seanmonstar/warp/blob/master/examples/dyn_reply.rs
async fn status_handler() -> Result<Box<dyn Reply>, Rejection> {
    Ok(Box::new(reply::json(&"Server is running")))
}

#[tokio::test]
async fn test_status() {
    let filter = status_filter();
    let result = warp::test::request()
        .path("/api/status")
        .reply(&filter)
        .await;
    assert_eq!(result.status(), 200, "{}", result.status());
}
