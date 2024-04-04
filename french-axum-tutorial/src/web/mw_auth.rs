use axum::{async_trait, RequestPartsExt};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::{Error, Res};
use crate::web::AUTH_TOKEN;


pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Res<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token_str = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token_str
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(|t| AuthToken::parse(t))
    {
        Ok(auth_token) => {
            Ok(Ctx::new(auth_token.user_id))
        },
        Err(e) => Err(e)
    };

    // remove the cookie if something went wrong, other than there being no cookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth(
    ctx: Res<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Res<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    
    ctx?;

    Ok(next.run(req).await)
}


// ctx extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Res<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Res<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}



struct AuthToken {
    user_id: u64,
    expiration: String,
    signature: String
}

impl AuthToken {
    /// parses a token of the format `user-[user-id].[expiration].[signature]`
    fn parse(token: String) -> Res<Self> {
        let (_whole, user_id, exp, sign) = regex_captures!(
            r#"^user-(\d+)\.(.+)\.(.+)"#,
            &token
        )
        .ok_or(Error::AuthFailTokenWrongFormat)?;

        let user_id = user_id.parse::<u64>()
            .map_err(|_| Error::AuthFailTokenWrongFormat)?;

        Ok(Self { user_id, expiration: exp.into(), signature: sign.into() })
    }
}