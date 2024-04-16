use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, body, Server, Method, StatusCode};
use reqwest::Client;
use serde_json::Value;
use std::convert::Infallible;
use anyhow::anyhow;
use base64;
use once_cell::sync::Lazy;
use std::env;

const DAPR_PORT: &str = "3500";
const SECRET_STORE_NAME: &str = "secret-store-k8s";
const SECRET_KEY_NAME: &str = "encryption-key-secret";
const APP_PORT: u16 = 8080;

static USE_STATIC_KEY: Lazy<bool> = Lazy::new(|| {
    env::args().any(|arg| arg == "--use-static-key")
});

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = ([127, 0, 0, 1], APP_PORT).into();
    let make_svc = make_service_fn(|_conn| {
        let service = service_fn(|req: Request<Body>| async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Ok(Response::new(Body::from("Secrets Crypt App Runs!"))),
                (&Method::POST, "/encrypt") => handle_encrypt(req).await,
                (&Method::POST, "/decrypt") => handle_decrypt(req).await,
                _ => Ok::<_, Infallible>(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap()),
            }
        });
        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn get_secret() -> Result<Vec<u8>, anyhow::Error> {
    if *USE_STATIC_KEY {
        println!("Fetching static secret...");
        let secret = "ZuwE9ofPpQhd0tiN0rIdf0ezkJCKwQV/XgXVOfj5nmc=";
        let secret_key = base64::decode(secret)?;
        Ok(secret_key)
    } else {
        println!("Fetching secret from Dapr...");
        let client = Client::new();
        let secret_url = format!("http://localhost:{}/v1.0/secrets/{}/{}", DAPR_PORT, SECRET_STORE_NAME, SECRET_KEY_NAME);
        let res = client.get(secret_url)
                        .send()
                        .await?
                        .text()
                        .await?;

        let secret: Value = serde_json::from_str(&res)?;
        let secret_key = secret["encryptionKey"].as_str().ok_or_else(|| 
            anyhow!("encryptionKey not found"))?.as_bytes().to_vec();
        let secret_key = base64::decode(secret_key)?;
        println!("Decoded key length: {}", secret_key.len());
        Ok(secret_key)
    }
}

fn get_nonce() -> [u8; 12] {
    // In a real-world scenario, you would generate a random nonce here.
    // The nonce needs to be the same for encryption and decryption and should be unique for each iteration.
    // For simplicity and demonstration, we're using a hardcoded nonce.
    *b"unique nonce" // 12 bytes
}

async fn handle_encrypt(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Encrypting data...");
    match get_secret().await {
        Ok(secret_key) => {
            let key = Key::from_slice(&secret_key); // 32 bytes
            let cipher = Aes256Gcm::new(key);

            let nonce_bytes = get_nonce();
            let nonce = Nonce::from_slice(&nonce_bytes);

            match body::to_bytes(req.into_body()).await {
                Ok(whole_body) => {
                    let data = whole_body.as_ref();

                    // Encrypting the data
                    match cipher.encrypt(&nonce, data) {
                        Ok(ciphertext) => Ok(Response::new(Body::from(ciphertext))),
                        Err(_) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Encryption failed"))
                            .unwrap()),
                    }
                },
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid request body"))
                    .unwrap()),
            }
        },
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to get secret"))
            .unwrap()),
    }
}

async fn handle_decrypt(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Decrypting data...");
    match get_secret().await {
        Ok(secret_key) => {
            let key = Key::from_slice(&secret_key); // 32 bytes
            let cipher = Aes256Gcm::new(key);

            let nonce_bytes = get_nonce();
            let nonce = Nonce::from_slice(&nonce_bytes);

            match body::to_bytes(req.into_body()).await {
                Ok(whole_body) => {
                    let data = whole_body.as_ref();

                    // Decrypting the data
                    match cipher.decrypt(nonce, data) {
                        Ok(plaintext) => Ok(Response::new(Body::from(plaintext))),
                        Err(_) => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Decryption failed"))
                            .unwrap()),
                    }
                },
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid request body"))
                    .unwrap()),
            }
        },
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to get secret"))
            .unwrap()),
    }
}
