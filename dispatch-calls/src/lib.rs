// proxy-wasm
use log::{debug, warn};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

// This is the initial entry point of the plugin.
proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);

    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyRootContext{}) });
}}

struct MyPlugin;

impl HttpContext for MyPlugin {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        match self.dispatch_http_call(
            "google",
            vec![
                (":method", "GET"),
                (":path", "/"),
                (":authority", "www.google.com"),
                (":scheme", "https"),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        ) {
            Ok(_) => Action::Pause,
            Err(e) => {
                warn!("Failed to dispatch_http_call: {:?}", e);
                Action::Pause
            }
        }
    }
}

impl Context for MyPlugin {
    fn on_http_call_response(&mut self, _token_id: u32, _: usize, body_size: usize, _: usize) {
        let headers = self.get_http_call_response_headers();
        debug!("on_http_call_response: {:?}", headers);

        let content_type = self.get_http_call_response_header("content-type").unwrap();
        debug!("content-type: {:?}", content_type);

        let server = self.get_http_call_response_header_bytes("server").unwrap();
        let server_str = String::from_utf8(server).unwrap();
        debug!("server: {:?}", server_str);

        let body = self.get_http_call_response_body(0, body_size).unwrap();
        let body_str = String::from_utf8(body).unwrap();
        debug!("body: {:?}", body_str);

        let trailer = self.get_http_call_response_trailers();
        debug!("trailer: {:?}", trailer);

        self.resume_http_request();

        // Do something with the response
    }
}

pub struct MyRootContext;

impl RootContext for MyRootContext {
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyPlugin {}))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for MyRootContext {}
