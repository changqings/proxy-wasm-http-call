use std::time;
use std::vec;

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_http_context(|_,_| -> Box<dyn HttpContext> { Box::new(HttpCall)});
}}

struct HttpCall;
impl HttpContext for HttpCall {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        match self.dispatch_http_call(
            "httpbin-test",
            vec![
                (":method", "GET"),
                (":path", "/uuid"),
                (":authority", "httpbin.org"),
                ("Content-Type", "application/x-www-form-urlencoded"),
            ],
            None,
            vec![],
            time::Duration::from_secs(10),
        ) {
            Ok(o) => info!("get ok, uuid = {}", o),
            Err(e) => info!("get err, {:?}", e),
        }

        Action::Pause
    }
}

impl Context for HttpCall {
    fn on_http_call_response(
        &mut self,
        token_id: u32,
        _num_headers: usize,
        body_size: usize,
        _num_trailers: usize,
    ) {
        let body = self.get_http_call_response_body(0, body_size);
        match body {
            Some(b) => info!(
                "token_id {} call response body = {}",
                token_id,
                String::from_utf8(b).unwrap()
            ),
            _ => info!("token_id {} call response none", token_id),
        }

        self.resume_http_request();
    }
}
