use axum::{middleware, response::Response, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tower_http::services::ServeDir;


pub mod error;
pub mod web;
pub mod model;
pub mod ctx;

use error::{Error, Res};
use model::ModelController;






async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper\n", "RES_MAPPER");

    res
}


#[tokio::main]
async fn main() -> Res<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    info!("tracing initialized");

    let mc = ModelController::new().await?;


    // let api_routes = web::routes_tickets::routes(mc.clone())
    //     .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    

    let api_routes = Router::new()
        .nest("/tickets", web::routes_tickets::routes(mc.clone()))
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let static_stuff = Router::new()
        .nest_service("/", ServeDir::new("dist"));

    let router = Router::new()
        .nest("/api", api_routes)
        .nest("/hello", web::routes_hello::routes())
        .nest("/login", web::routes_login::routes())
        .merge(static_stuff)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(), 
            web::mw_auth::mw_ctx_resolver
        ))
        .layer(CookieManagerLayer::new());
    // .layer(TraceLayer::new_for_http())

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
