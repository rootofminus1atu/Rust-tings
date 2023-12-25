use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};



#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello wooorld")})
    );


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("LISTENING ON {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()
}
