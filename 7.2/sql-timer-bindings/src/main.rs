use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use reqwest;
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

const POSTGRES_TABLE_NAME: &str = "counter_table";
const POSTGRES_BINDING_NAME: &str = "postgres-binding-counter";
const DAPR_HTTP_PORT: &str = "DAPR_HTTP_PORT";
const YOUR_SERVER_PORT: u16 = 8080;

async fn handle_dapr_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri = req.uri().clone();
    let path = uri.path();

    let dapr_http_port: u16 = std::env::var(DAPR_HTTP_PORT).unwrap_or_else(|_| "3500".to_string()).parse().unwrap();
    let dapr_url = format!("http://localhost:{}/v1.0/bindings/{}", dapr_http_port, POSTGRES_BINDING_NAME);

    if path == "/cron-binding" {
        println!("Cron binding invoke - URI path is /cron-binding");

        let client = reqwest::Client::new();

        // Update the counter using the Postgres binding
        let update_query = json!({
            "operation": "exec",
            "metadata": {
                "sql": format!(
                    "INSERT INTO {} (id, counter) VALUES (1, 1) ON CONFLICT (id) DO UPDATE SET counter = {}.counter + 1;",
                    POSTGRES_TABLE_NAME, POSTGRES_TABLE_NAME
                )
            }
        });

        match client.post(&dapr_url)
            .json(&update_query)
            .send()
            .await {
            Ok(_) => println!("Cron job triggered. Counter incremented in Postgres."),
            Err(e) => eprintln!("Error invoking binding: {}", e),
        }

        // Lookup the current counter value using the Postgres binding
        let select_query = json!({
            "operation": "query",
            "metadata": {
                "sql": format!("SELECT counter FROM {} WHERE id = $1", POSTGRES_TABLE_NAME),
                "params": "[1]"
            }
        });

        match client.post(&dapr_url)
            .json(&select_query)
            .send()
            .await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                println!("Counter: {}", body);
            },
            Err(e) => eprintln!("Error fetching current counter: {}", e),
        }

        Ok(Response::new(Body::from("Cron job processed")))
    } else {
        println!("No cron binding invoke - URI: {:?}", uri);

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Delay the start of the server just to ensure that Dapr sidecar is ready
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let server_addr = SocketAddr::from_str(&format!("127.0.0.1:{}", YOUR_SERVER_PORT))?;

    let server = Server::bind(&server_addr).serve(make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| {
            handle_dapr_request(req)
        }))
    }));

    println!("Listening on http://{}", server_addr);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    
    Ok(())
}
