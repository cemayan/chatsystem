mod global;
mod api;

use global::CONFIGS;
use logger::logger_init;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    logger_init(CONFIGS.logger.level.to_string());

    let router = api::router::create_app().await;
    let config = CONFIGS.to_owned();

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(config.server.ws.unwrap().addr)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .expect("Failed to start server")
}

