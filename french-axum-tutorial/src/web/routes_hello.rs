use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { Html("Hello wooorld")}))
        .route("/2", get(handler_hello_query))
        .route("/3/:name", get(handler_hello_path))
}

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