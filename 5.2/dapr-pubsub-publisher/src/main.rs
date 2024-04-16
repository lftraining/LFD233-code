use reqwest::Client;
use serde_json::json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

// The topic we want to publish messages to
const DAPR_PUBSUB_TOPIC: &str = "status-topic";
// The time to wait between publishing messages
const TIMEOUT: u64 = 3;
// Dapr Pub Sub component
const DAPR_PUBSUB_COMPONENT: &str = "pubsub";

#[tokio::main]
async fn main() {
    let dapr_port: u16 = env::var("DAPR_HTTP_PORT")
        .unwrap_or_else(|_| "3500".to_string())
        .parse()
        .expect("Invalid DAPR port");

    let client = Client::new();
    let dapr_url = format!("http://localhost:{}/v1.0/publish/{}/{}", dapr_port, DAPR_PUBSUB_COMPONENT, DAPR_PUBSUB_TOPIC);

    loop {
        if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let message = json!({
                DAPR_PUBSUB_TOPIC: format!("'Hello!' Sent by the publisher application at {}", time.as_secs())
            });

            match client.post(dapr_url.clone())
                        .json(&message)
                        .send()
                        .await {
                Ok(response) => {
                    println!("Response: {:?}", response);
                    if response.status().is_success() {
                        println!("Published message: {}", message);
                    } else {
                        println!("Failed to publish message: {:?}", response.status());
                    }
                },
                Err(e) => println!("Failed to send request: {:?}", e),
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(TIMEOUT)).await;
    }
}
