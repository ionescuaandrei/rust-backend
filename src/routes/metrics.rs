
use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

use crate::metrics::Kind;

pub fn register() -> Router {
  Router::new()
      .route("/", get(get_metrics))
      .route("/{kind}", get(get_metric))
}

async fn get_metrics() -> impl IntoResponse {
  "Implement the get_metrics endpoint"
}

async fn get_metric(Path(kind): Path<Kind>) -> impl IntoResponse {
    kind.to_string();
  }