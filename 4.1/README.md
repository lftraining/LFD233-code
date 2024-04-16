# Using Dapr

In this lab, we do not have any code and just use the terminal! We will start dapr and get familiar with the dapr sidecar.

All applications that are used throughout the entire course are listed under [Installs](https://github.com/lftraining/LFD233-code/?tab=readme-ov-file#installs).

**How complete the lab**:
1. Discover the CLI: `dapr -h`
2. Start the default Dapr setup: `dapr init`
3. Discover containers: `docker ps`
4. Discover created Dapr resources: `ls ~/.dapr`
5. Discover created Dapr resources using the Dapr dashboard: `dapr dashboard`
6. Start the sidecar: `dapr run --app-id blankapp --dapr-http-port 3000`
7. Use the sidecar to store information in the statestore component: `curl -X POST -H "Content-Type: application/json" -d '[{ "key": "name", "value": "Bruce Wayne"}]' http://localhost:3000/v1.0/state/statestore`
8. Retrieve stored information using the Dapr sidecar: `curl http://localhost:3000/v1.0/state/statestore/name`

Done!