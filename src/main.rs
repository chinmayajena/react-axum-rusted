#![allow(unused)]

pub use self::error::{ Error, Result };

use std::net::SocketAddr;
use axum::{ Router, middleware };
use axum::extract::{ Query, Path };
use axum::routing::{ get, get_service };
use axum::response::{ Html, IntoResponse, Response };
use model::ModelController;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets
        ::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->>>>>>>>>>>Listening on {addr}\n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello)).route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -  {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("hello <strong>{name} We did it</strong>")) // Be careful of the semicolon here
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -  {name:?}", "Handler");
    Html(format!("hello <strong>{name} We did it</strong>"))
}
