use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!(
                {
                    "message": "Hello, world!",
                }
            )
            .to_string()
            .into(),
        )?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}
