use std::fs::{File, OpenOptions};
use std::io::Write;

use axum::{extract::Query, http::{header::{ACCEPT, AUTHORIZATION, ORIGIN}, HeaderValue, Method, Request}, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir, trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer}};
use tracing::info;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::EnvFilter;

pub mod html_extractor;

/*
This works under the assumptions:
- a timetable will always start from 9:00
- a hour will always have a font color of #FFFFFFF
- a single lesson will never be shorter than 1 hour
*/

#[derive(Debug, Deserialize)]
struct TimetableParams {
    student_group: String,
    which_week: i32
}

const URL_BASE: &str = "http://timetables.itsligo.ie:81/reporting/individual;student+set;id;";
const CRLF: &str = "%0D%0A";  // why is this included in all urls?
const DAYS_LINE: &str = "&days=1-5";
const PERIODS_LINE: &str = "&periods=3-20";

fn construct_url(student_group: &str, which_week: usize) -> String {
    format!("
        {URL_BASE}{student_group}{CRLF}?t=student+set+individual{DAYS_LINE}&weeks={which_week}{PERIODS_LINE}&template=student+set+individual",
    )
}

async fn request_timetable_html(student_group: &str, which_week: usize) -> Result<(), Box<dyn std::error::Error>> {

    let url = construct_url(student_group, which_week);

    let response = reqwest::get(&url)
        .await?
        .text()
        .await?;

    // write to file called "write-here.html"
    println!("{}", response);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("write-here.html")?;
    file.write_all(response.as_bytes())?;

    Ok(())
}

async fn get_timetable(Query(params): Query<TimetableParams>) -> impl IntoResponse {
    let TimetableParams { student_group, which_week } = params;

    let url = construct_url(&student_group, which_week as usize);
    info!(url);

    Json(
        json!({
            "processed": student_group,
            "lol": which_week
        })
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    tracing::info!("tracing is initialized");

    // let cors = CorsLayer::new()
    //     .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     // .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
    //     .allow_origin(Any);
    tracing::info!("hello");

    let router = Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .route("/api/timetable", get(get_timetable))
        
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
        );

    Ok(router.into())
}
