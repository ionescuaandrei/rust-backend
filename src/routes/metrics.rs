use axum::Json;
use axum::{Router, extract::Path, response::IntoResponse, routing::get};
use crate::metrics::init;

use crate::metrics::{Kind, Summary, System, Process, Memory, Cpu, Disk};

pub fn register() -> Router {
  Router::new()
      .route("/", get(get_metrics))
      .route("/{kind}", get(get_metric))
      
}

async fn get_metrics() -> impl IntoResponse {
    let mut sys = init().await;
    Json(serde_json::to_value(&Summary::generate(&mut sys)).unwrap())

}

async fn get_metric(Path(kind): Path<Kind>) -> impl IntoResponse {
    let mut sys = init().await;
    match kind {
        Kind::System => Json(serde_json::to_value(System::generate()).unwrap()),
        Kind::Process => Json(serde_json::to_value(Process::generate(&mut sys)).unwrap()),
        Kind::Memory => Json(serde_json::to_value(Memory::generate(&mut sys)).unwrap()),
        Kind::Cpu => Json(serde_json::to_value(Cpu::generate(&mut sys)).unwrap()),
        Kind::Disk => Json(serde_json::to_value(Disk::generate()).unwrap()),
    }
}