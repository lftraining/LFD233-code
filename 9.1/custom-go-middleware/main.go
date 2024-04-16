package main

import (
	"fmt"

	"github.com/http-wasm/http-wasm-guest-tinygo/handler"
	"github.com/http-wasm/http-wasm-guest-tinygo/handler/api"
)

func main() {
	handler.HandleRequestFn = handleRequest
}

func handleRequest(req api.Request, resp api.Response) (next bool, reqCtx uint32) {
	// Set a custom header which get's checked by the http-header-check application
	req.Headers().Set("X-Test-Header", "HeaderValue")

	handler.Host.Log(api.LogLevelInfo, fmt.Sprintf("Middleware processing URI: %s and set X-Test-Header", req.GetURI()))

	next = true
	return
}
