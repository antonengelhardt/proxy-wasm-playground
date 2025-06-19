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
        let headers = self.get_http_request_headers();
        let path = self.get_http_request_header(":path").unwrap_or_default();

        debug!("on_http_request_headers: {:?}", headers);
        debug!("on_http_request_headers: path: {:?}", path);

        

        Action::Continue
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
