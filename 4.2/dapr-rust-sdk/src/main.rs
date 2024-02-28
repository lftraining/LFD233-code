use hyper::{Method, Request, Response};
use bytes::Bytes;
use anyhow::{Result, Error};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::service::service_fn;
use std::{convert::Infallible, net::SocketAddr};
use serde_json::Value;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper_util::rt::tokio::TokioIo;

const PORT : u16 = 8081;
const STATE_STORE_NAME: &str = "statestore";
const STATE_STORE_INCREMENT_KEY: &str = "counter";

async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
    let addr = format!("https://127.0.0.1:{}", port);

    // Create the client
    // this could also be done just once in the beggining of the main function
    let mut c = dapr::Client::<dapr::client::TonicClient>::connect(addr).await?;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let current_value_result = c.get_state(STATE_STORE_NAME, STATE_STORE_INCREMENT_KEY, None).await;
            match current_value_result {
                Ok(current_value) => {
                    let new_value_result = serde_json::from_slice::<Value>(&current_value.data);
                    let new_value = match new_value_result {
                        Ok(val) => val.as_i64().unwrap_or(0) + 1,
                        Err(_) => 0,
                    };

                    let serialized_new_value = serde_json::to_vec(&new_value).unwrap();
                    match c.save_state(STATE_STORE_NAME, vec![(STATE_STORE_INCREMENT_KEY, serialized_new_value)]).await {
                        Ok(_) => {
                            println!("Counter incremented to: {}", new_value);
                            Ok(Response::new(full(format!("Counter incremented to: {}", new_value))))
                        },
                        Err(e) => Ok(Response::new(full(format!("Error: {}", e))))
                    }
                },
                Err(e) => Ok(Response::new(full(format!("Error retrieving current state: {}", e))))
            }
        },
        _ => Ok(Response::new(full("Not Found")))
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|_: Infallible| -> Error { unreachable!() })
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("PageCounter App started. Wait for Dapr sidecar to start ...");
    std::thread::sleep(std::time::Duration::new(2, 0));

    // Get the Dapr port and create a connection
    let port: u16 = std::env::var("DAPR_GRPC_PORT")?.parse()?;
    let addr = format!("https://127.0.0.1:{}", port);

    // Create the client
    let _ = dapr::Client::<dapr::client::TonicClient>::connect(addr).await?;
    println!("Dapr is healthy!");

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
