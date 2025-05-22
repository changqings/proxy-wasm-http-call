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
curl 127.0.0.1:8080/headers

# things not solve, this two req not the same
wasm log
```
wasm log http_call_wasm http_call_wasm: get ok, uuid = 4
wasm log http_call_wasm http_call_wasm: token_id 4 call response body = {
  "headers": {
    "Content-Type": "application/x-www-form-urlencoded", 
    "Host": "httpbin.org", 
    "X-Amzn-Trace-Id": "Root=1-682f5bab-3e59708316f0089e7e2865f4", 
    "X-Envoy-Expected-Rq-Timeout-Ms": "10000", 
    "X-Envoy-Internal": "true"
  }
}

 ^Ccaught SIGINT
shutting down server instance
Notifying 0 callback(s) with completion.
main dispatch loop exited
exiting
```
curl log
```
shenchangqing@shenchangqing:~/github/wasm-http-body$ curl -v http://localhost:8082/headers
* Host localhost:8082 was resolved.
* IPv6: ::1
* IPv4: 127.0.0.1
*   Trying [::1]:8082...
* connect to ::1 port 8082 from ::1 port 46520 failed: 连接被拒绝
*   Trying 127.0.0.1:8082...
* Connected to localhost (127.0.0.1) port 8082
* using HTTP/1.x
> GET /headers HTTP/1.1
> Host: localhost:8082
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< date: Thu, 22 May 2025 17:18:55 GMT
< content-type: application/json
< content-length: 237
< server: envoy
< access-control-allow-origin: *
< access-control-allow-credentials: true
< x-envoy-upstream-service-time: 614
< proxy-wasm: hello
<
{
  "headers": {
    "Accept": "*/*",
    "Host": "localhost",
    "Tv": "tcl",
    "User-Agent": "curl/8.12.1",
    "X-Amzn-Trace-Id": "Root=1-682f5c7f-072a8dbb1dd0efd5680a0d14",
    "X-Envoy-Expected-Rq-Timeout-Ms": "15000"
  }
}
* Connection #0 to host localhost left intact

```


