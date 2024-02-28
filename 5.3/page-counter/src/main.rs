use hyper::{Method, Request, Response, StatusCode, Server, Body};
use reqwest::Client;
use hyper::service::{make_service_fn, service_fn};
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

const PORT : u16 = 8081;
const STATE_STORE_NAME: &str = "statestore";
const STATE_STORE_INCREMENT_KEY: &str = "counter";
const STATE_STORE_PORT: u16 = 3500;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let client = reqwest::Client::new();
            let new_counter_value = increment_state(&client, STATE_STORE_INCREMENT_KEY).await?;
            let response_body = format!("Counter incremented to: {}", new_counter_value);
            Ok(Response::new(Body::from(response_body)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

/// Increment the counter in the Dapr state store.
async fn increment_state(client: &Client, key: &str) -> Result<i32, anyhow::Error> {
    let state_url = format!("http://localhost:{}/v1.0/state/{}", STATE_STORE_PORT, STATE_STORE_NAME);

    let current_value: i32 = match client.get(format!("{}/{}", state_url, key))
        .send()
        .await?
        .json::<i32>()
        .await {
            Ok(value) => value,
            Err(_) => 0, // Using 0 as a default value if the key does not exist
        };

    let new_value = current_value + 1;
    let state = json!([{"key": key, "value": new_value}]);

    client.post(state_url)
        .json(&state)
        .send()
        .await?
        .error_for_status()?;
    Ok(new_value)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("PageCounter App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let make_svc = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });
    let server = Server::bind(&addr).serve(make_svc);
    dbg!(format!("Listening on http://{}", addr));
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
