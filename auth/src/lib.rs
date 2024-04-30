// proxy-wasm
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use log::debug;

// This is the initial entry point of the plugin.
proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);

    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyRootContext{}) });
}}

struct MyPlugin;

impl HttpContext for MyPlugin {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let auth = self
            .get_http_request_header("Authorization")
            .unwrap_or_default();

        if auth == "I am authorized" {
            debug!("Authorized");
            self.send_http_response(200, vec![], Some(b"Authorized"));
        } else {
            debug!("Unauthorized");
            self.send_http_response(
                403,
                vec![("location", "https://www.google.com")],
                Some(b"Unauthorized"),
            );
        }
        return Action::Pause;
    }
}

impl Context for MyPlugin {}

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
