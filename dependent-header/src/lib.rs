use log::info;
// proxy-wasm
use log::{warn,debug};
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
        // Dispatch an HTTP call to www.google.com
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

        let _body = self
            .get_http_call_response_body(0, body_size)
            .unwrap_or_default();

        let google_server_name = self
            .get_http_call_response_header("server")
            .unwrap_or("unknown".to_string());
        debug!("google server name: {}", google_server_name);

        // Convert the body to a string and log it.
        let parsed_body = String::from_utf8_lossy(&_body);

        let contains_google_apis = parsed_body.contains("googleapis");

        if contains_google_apis {
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
                Ok(_) => {
                    info!("dispatched another http call")
                },
                Err(e) => {
                    warn!("Failed to second dispatch_http_call: {:?}", e);
                }
            }
        }

        let contains_some_other_weird_stuff = parsed_body.contains("some_other_weird_stuff");

        self.add_http_request_header("x-contains-google-apis", &contains_google_apis.to_string());
        self.add_http_request_header(
            "x-contains-some-other-weird-stuff",
            &contains_some_other_weird_stuff.to_string(),
        );
        self.add_http_request_header("x-google-server-name", &google_server_name);

        self.resume_http_request();
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
