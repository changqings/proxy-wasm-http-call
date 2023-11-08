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
        let req = self.dispatch_http_call(
            "httpbingo",
            vec![
                (":method", "GET"),
                (":path", "/uuid"),
                (":authority", "httpbingo.org"),
                ("scheme", "https"),
            ],
            None,
            vec![],
            time::Duration::from_secs(5),
        );
        match req {
            Ok(o) => info!("get ok, uuid = {}", o),
            Err(e) => info!("get err {:?}", e),
        }

        Action::Continue
    }
}

impl Context for HttpCall {}
