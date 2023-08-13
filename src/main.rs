#![allow(unused)]

use std::net::SocketAddr;
use axum::Router;
use axum::extract::Query;
use axum::routing::get;
use axum::response::{ Html, IntoResponse };
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->>>>>>>>>>>Listening on {addr}\n");

    axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -  {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("World!");
    println!("{}", name);
    Html(format!("hello <strong>{name} We did it</strong>")) // Be careful of the semicolon here
}
