pub mod routes;
pub mod metrics;
use axum;

use routes::app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(
        "Server running on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app()).await.unwrap();
}