use std::ops::Add;

use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use reqwest;
use serde::Deserialize;
use tower_http::services::ServeDir;

/*
This works under the assumptions:
- a timetable will always start from 9:00
- a hour will always have a font color of #FFFFFFF
- a single lesson will never be shorter than 1 hour
*/

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize)]
struct AddParams {
    search_term: String
}

async fn add(Query(params): Query<AddParams>) -> impl IntoResponse {
    let search_term = params.search_term;

    

    Html("Hello I got nothing")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/add", get(add))
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}
