mod app;
mod di;
mod error;
mod user;

use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = app::create_app();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
