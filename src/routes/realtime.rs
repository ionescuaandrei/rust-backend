use axum::{response::{sse::{Event, KeepAlive}, IntoResponse, Sse}, routing::get, Json, Router};
use std::time::Duration;
use tokio_stream::{StreamExt, wrappers::IntervalStream};

use crate::metrics::init;

pub fn register() -> Router {
    Router::new()
    .route("/", get(realtime_metrics))
}

async fn realtime_metrics() -> impl IntoResponse {
    let mut sys = init().await;
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
        let metrics = crate::metrics::Summary::generate(&mut sys);
        let event = Event::default()
            .data(serde_json::to_string(&metrics).unwrap_or_default());
        Ok::<_, serde_json::Error>(event)
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(5)))
}