use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;


pub type Res<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]

pub enum Error {
    LoginFail,

    // model error
    TicketDeleteFailIdNotFound { id: u64 },

    // auth error
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}