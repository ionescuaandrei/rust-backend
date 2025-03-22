use axum::{routing::get, Router};

mod healthcheck;
mod metrics;
mod realtime;

pub fn app() -> Router {
    Router::new()
    .nest("/healthcheck",
     healthcheck::register())
    .nest("/metrics", metrics::register())
    .nest("/realtime", realtime::register() )
}