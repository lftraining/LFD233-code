use hyper::{Method, Request, Response};
use bytes::Bytes;
use anyhow::{Result, Error};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::service::service_fn;
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper_util::rt::tokio::TokioIo;
use serde_json::json;

// The port the application will listen on
const PORT : u16 = 8081;
// Topic to subscribe to
const DAPR_PUBSUB_TOPIC: &str = "status-topic";
// Subscription route to receive messages
const DAPR_PUBSUB_ROUTE: &str = "/status";
// PubSub component name
const DAPR_PUBSUB_COMPONENT: &str = "pubsub";

async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/dapr/subscribe") => {
            let subscriptions = json!([
                {
                    "pubsubname": DAPR_PUBSUB_COMPONENT,
                    "topic": DAPR_PUBSUB_TOPIC,
                    "route": DAPR_PUBSUB_ROUTE,
                }
            ]);
            let subscriptions = serde_json::to_string(&subscriptions).unwrap();
            
            println!("Dapr pub/sub is subscribed to: {}", subscriptions);
            Ok(Response::new(full(subscriptions)))
        },
        (&Method::POST, DAPR_PUBSUB_ROUTE) => {
            let body_bytes = req.into_body().boxed().collect().await?;
            let body_str = String::from_utf8(body_bytes.to_bytes().to_vec()).unwrap();
            
            let json_value: serde_json::Value = serde_json::from_str(&body_str).unwrap();
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());

            Ok(Response::new(full("Message processed")))
        },
        _ => {
            let error_msg = "Unsupported request".to_string();
            println!("{}", error_msg);
            Ok(Response::new(full(error_msg)))
        },
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|_: Infallible| -> Error { unreachable!() })
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("PUB SUB Subscriber App started. Wait for Dapr sidecar to start ...");
    std::thread::sleep(std::time::Duration::new(1, 0));

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