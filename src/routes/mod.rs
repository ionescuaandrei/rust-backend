use axum::{routing::get, Router};

mod healthcheck;
mod metrics;

pub fn app() -> Router {
    Router::new()
        .nest("/healthcheck", healthcheck::register())
        .nest("/metrics", metrics::register())
}