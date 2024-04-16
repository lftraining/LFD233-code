# HTTP Header Check

This project implements an http server which checks if a header is set `const HEADER_KEY: &str = "X-Test-Header";`. If that is not the case it will return an `StatusCode::BAD_REQUEST`. This project code is used together with the `custom-middleware` project to illustrate how a middleware can be used to update headers of incoming http requests. 

**How to use independently**: 
1. if not done before, run `rustup target add wasm32-wasi` to be able to compile to WASI format.
1. Build Rust application to WASM bin `cargo build --target wasm32-wasi`
2. Run WASM binary using WasmEdge: `wasmedge target/wasm32-wasi/debug/http-header-check.wasm`
3. Call the application:
   1. With Header set: `curl -v http://localhost:8080/ -H "X-Test-Header: xyz"`
   2. Without Header set: `curl -v http://localhost:8080/`
