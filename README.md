# proxy-wasm-http-call

rust = 1.87.0
envoy 1.28.0

# rust target

rust target add wasm32-wasip1

# build

make build

# run

make run

# curl

```bash
curl 127.0.0.1:8080/headers
```
