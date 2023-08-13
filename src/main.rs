#![allow(unused)]

use axum::Router;
use axum::routing::get;
use axum::response::Html;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("hello <strong>World!!!</strong>") })
    );

    let addr = Sock
}
