use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use tracing::info;
use serde::Deserialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

// old way:
// let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
// println!("LISTENING ON {}", addr);
// axum::Server::bind(&addr)
//     .serve(routes_hello.into_make_service())
//     .await
//     .unwrap()


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}


async fn handler_hello_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("Hello, <strong>{}</strong>", name))
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, <i>{}</i>", name))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    info!("tracing initialized");


    let hello_routes = Router::new()
        .route("/", get(|| async { Html("Hello wooorld")}))
        .route("/2", get(handler_hello_query))
        .route("/3/:name", get(handler_hello_path));

    let api_routes = Router::new()
        .nest("/hello", hello_routes);

    let static_stuff = Router::new()
        .nest_service("/", ServeDir::new("dist"));

    let router = Router::new()
        .nest("/api", api_routes)
        .merge(static_stuff)
        .layer(TraceLayer::new_for_http());

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
