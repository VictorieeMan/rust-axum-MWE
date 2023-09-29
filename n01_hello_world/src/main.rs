//Created: 2023-09-29
//Sourced from: https://web.archive.org/web/20230924001656/https://docs.rs/axum/latest/axum/
//Minimal Working Example for Axum, a "Hello World!" server.

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// cargo run
// Open browser at localhost:3000