# Page counter application

In this lab, we will develop an application to communicate with the Dapr sidecar. In the previous lab we used the terminal to send HTTP requests, this can also be done in an application. The application will be served using WasmEdge.

**How complete the lab**:
1. Naviagte to the `page-counter` folder: `cd page-counter`
2. Build the application in a WASI compatible version format: `cargo build --target wasm32-wasi`
3. Start Dapr: `dapr init`
4. Run the application alongside a Dapr sidecar container: `dapr run --app-id page-counter --app-port 8081 --dapr-http-port 3500 -- wasmedge target/wasm32-wasi/debug/page-counter.wasm`
5. Start the Dapr dashboard to observe the deployments: `dapr dashboard`
6. Test the application by calling the endpoint a few times and incrementing the counter: `curl http://0.0.0.0:8081`
7. Communicate with the Dapr sidecar directly to retrive the current page counter value: `curl http://localhost:3500/v1.0/state/statestore/counter`

Done!