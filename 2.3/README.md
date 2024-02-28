# Deploying a Rust Web Service with WasmEdge

This lab focuses on deploying a simple Rust web service using hyper with [WasmEdge](https://wasmedge.org/). WasmEdge is a lightweight, high-performance WebAssembly (WASM) runtime, ideal for edge computing and decentralized cloud applications.

**How complete the lab**:
1. Naviagte to the `hyper-wasi-server` folder: `cd hyper-wasi-server`
2. Build the application in a WASI compatible version format: `cargo build --target wasm32-wasi`
3. Run the binary using WasmEdge: `wasmedge target/wasm32-wasi/debug/dapr_wasm_02_wasmedge.wasm`
4. Naviagte to your browser or use the CLI to check that the application is working: `curl http://127.0.0.1:3000/`

Done!