# Wasm output binding
https://docs.dapr.io/reference/components-reference/supported-bindings/wasm/

```bash
cd components/dapr-wasm-binding 
cargo build --target wasm32-wasi --release
cd -
dapr run --app-id wasm --dapr-http-port 3500 --resources-path components &
curl -X POST http://localhost:3500/v1.0/bindings/wasm -d'
{
  "operation": "execute"
}'
```
Expected output: `Hello, binding!`


TODO ...