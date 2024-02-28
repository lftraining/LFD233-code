use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use reqwest;
use serde_json::Value;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::Mutex;
use std::sync::Arc;

const DAPR_PORT: &str = "3500";
const CONFIG_STORE_NAME: &str = "configstore";
const APP_PORT: u16 = 8080;
const CONFIG_KEY_WEBSITE_URL: &str = "website_url";
const CONFIG_KEY_INTERVAL_SECONDS: &str = "interval_seconds";

#[derive(Debug, Clone)]
struct AppConfig {
    website_url: String,
    interval_seconds: u64,
}

#[tokio::main]
async fn main() {
    let config = Arc::new(Mutex::new(AppConfig {
        website_url: String::new(),
        interval_seconds: 0,
    }));

    // Fetch the initial configuration
    if let Err(e) = fetch_initial_configuration(config.clone()).await {
        eprintln!("Failed to fetch and subscribe to configuration: {}", e);
    }

    // Subscribe to future configuration updates
    let subscribe_url = format!("http://localhost:{}/v1.0/configuration/{}/subscribe", DAPR_PORT, CONFIG_STORE_NAME);
    let client = reqwest::Client::new();
    match client.get(&subscribe_url).send().await {
        Ok(_) => println!("Subscribed to configuration updates"),
        Err(e) => eprintln!("Failed to subscribe to configuration updates with error: {}", e),
    }

    // Spawn the HTTP server as an async task and link it up to the listen_for_config_changes function
    let addr = SocketAddr::from(([0, 0, 0, 0], APP_PORT));
    let config_clone_for_server = config.clone();
    let make_svc = make_service_fn(move |_conn| {
        let config = config_clone_for_server.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                listen_for_config_changes(req, config.clone())
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    tokio::spawn(async move {
        println!("Listening on http://{}", addr);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });

    // Main loop to send health checks to the configured website
    loop {
        let (website_url, interval_seconds) = {
            let config_guard = config.lock().await;
            (config_guard.website_url.clone(), config_guard.interval_seconds)
        };

        if let Err(e) = health_check_website(&website_url).await {
            eprintln!("Failed to health check website: {}", e);
        } else {
            println!("Website {} is responding, waiting {} seconds for the next health check", website_url, interval_seconds);
        }

        sleep(Duration::from_secs(interval_seconds)).await;
    }
}


async fn fetch_initial_configuration(config: Arc<Mutex<AppConfig>>) -> Result<(), Box<dyn std::error::Error>> {
    let dapr_url = format!("http://localhost:{}/v1.0/configuration/{}/?keys={},{}",
        DAPR_PORT, CONFIG_STORE_NAME, CONFIG_KEY_WEBSITE_URL, CONFIG_KEY_INTERVAL_SECONDS);
    let response = reqwest::get(&dapr_url).await?;
    let config_data: Value = response.json().await?;
    println!("Received initial configuration: {:?}", config_data);

    // Extract and update global configuration
    let mut config_guard = config.lock().await;
    if let Some(new_website_url) = config_data[CONFIG_KEY_WEBSITE_URL]["value"].as_str() {
        config_guard.website_url = new_website_url.to_string();
        println!("Initial fetch website_url set to {}", new_website_url);
    }
    if let Some(new_interval_str) = config_data[CONFIG_KEY_INTERVAL_SECONDS]["value"].as_str() {
        if let Ok(new_interval) = new_interval_str.parse::<u64>() {
            config_guard.interval_seconds = new_interval;
            println!("Initial fetch interval_seconds set to {}", new_interval);
        }
    }

    Ok(())
}

async fn listen_for_config_changes(req: Request<Body>, config: Arc<Mutex<AppConfig>>) -> Result<Response<Body>, Infallible> {
    println!("Received a request.");

    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap_or_else(|_| hyper::body::Bytes::new());
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap_or_else(|_| String::from("(non-UTF8 data)"));
    println!("Received request body: {}", body_str);

    // Parse the request body as JSON
    if let Ok(json_body) = serde_json::from_str::<Value>(&body_str) {
        if let Some(items) = json_body["items"].as_object() {
            let mut config_guard = config.lock().await;

            // Update website_url if present
            if let Some(website_url) = items.get(CONFIG_KEY_WEBSITE_URL).and_then(|v| v["value"].as_str()) {
                config_guard.website_url = website_url.to_string();
                println!("Updated website_url to {}", website_url);
            }

            // Update interval_seconds if present
            if let Some(interval_seconds) = items.get(CONFIG_KEY_INTERVAL_SECONDS).and_then(|v| v["value"].as_str()) {
                if let Ok(interval) = interval_seconds.parse::<u64>() {
                    config_guard.interval_seconds = interval;
                    println!("Updated interval_seconds to {}", interval);
                }
            }
        }
    }

    Ok(Response::new(Body::from("Configuration updated if necessary")))
}

async fn health_check_website(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            println!("Website {} responded with status: {}", url, response.status());
        }
        Err(e) => {
            eprintln!("Failed to send HTTP request to {}: {}", url, e);
        }
    }
    Ok(())
}