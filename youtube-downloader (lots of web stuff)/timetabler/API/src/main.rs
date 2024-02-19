use axum::{extract::Query, http::{header::{ACCEPT, AUTHORIZATION, ORIGIN}, HeaderValue, Method, Request}, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir, trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer}};
use tracing::{level_filters::LevelFilter, Level};
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

    
    Json(
        json!({
            "processed": search_term
        })
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // let cors = CorsLayer::new()
    //     .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     // .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
    //     .allow_origin(Any);
    tracing::info!("hello");

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/api/add", get(add))
        .nest_service("/dist", ServeDir::new("dist"))
        // .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                tracing::info_span!(
                    "http_request",
                    "{} {}",
                    request.method(),
                    request.uri()
                )
            }),
        )
        ;

    Ok(router.into())
}
