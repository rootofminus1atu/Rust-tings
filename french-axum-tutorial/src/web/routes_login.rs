use crate::{web, Error, Res};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(api_login))
}


async fn api_login(cookies: Cookies ,payload: Json<LoginPayload>) -> Res<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "u1" || payload.pwd != "p1" {
        return Err(Error::LoginFail);
    }

    // todo: cookies generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}
