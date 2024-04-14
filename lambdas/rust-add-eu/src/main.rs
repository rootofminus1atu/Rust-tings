
use lambda_http::{http::StatusCode, run, service_fn, tracing, Error, IntoResponse, Request, RequestPayloadExt, Response};
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct MyPayload {
    a: i32, 
    b: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct MyResponse {
    result: i32
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful info from the request
    println!("event: {:?}", event);

    let body = event.payload::<MyPayload>()?.ok_or("Empty body bruh")?;
    let MyPayload { a , b } = body;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({
            "result": a + b,
          }).to_string())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

// cargo lambda deploy --region eu-west-1
