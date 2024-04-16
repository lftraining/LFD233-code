use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

const PORT: u16 = 8080;
const HEADER_KEY: &str = "X-Test-Header";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("HTTP Header Check App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let make_svc = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    println!("Received request for {}", req.uri().path());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/check-header") => {
            if req.headers().contains_key(HEADER_KEY) {
                println!("Request OK: '{}' header is present.", HEADER_KEY);
                let headers_list = req.headers().iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
                    .collect::<Vec<_>>().join("\n");

                let response = Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(headers_list))
                    .expect("Failed to construct valid response");
                
                Ok(response)
            } else {
                println!("Request NOT OK: '{}' header is missing.", HEADER_KEY);
                let error_message = "Error: Middleware failed to set the header.";
                let response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(error_message))
                    .expect("Failed to construct error response");
                
                Ok(response)
            }
        }
        _ => {
            let not_found = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .expect("Failed to construct not found response");
            
            Ok(not_found)
        }
    }
}
