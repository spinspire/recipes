use std::net::SocketAddr;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("Hello {}", "world");
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Param {
    name: Option<String>,
}

async fn hello(Query(param): Query<Param>) -> String {
    // (StatusCode::OK, format!("Hello {}!", name)).into_response()
    // (StatusCode::OK, "yo").into_response()
    // let name = match param.name {
    //     Some(name) => name,
    //     None => "Stranger".to_string(),
    // };
    format!("Hello {}!", param.name.unwrap_or("Stranger".to_string()))
}
