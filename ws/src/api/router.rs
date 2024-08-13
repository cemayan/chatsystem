use crate::api::{health, ws};
use axum::Router;
use tower_http::trace;

pub async fn create_app() -> Router {
    Router::new()
        .merge(Router::new().nest(
            "/api/v1/health",
            Router::new()
                .merge(health::create_route()),
        ))
        .merge(Router::new().nest(
            "/ws",
            Router::new()
                .merge(ws::create_route()),
        ))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(false))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
}