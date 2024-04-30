// proxy-wasm
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use serde_json;

use log::debug;

// This is the initial entry point of the plugin.
proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);

    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyRootContext{}) });
}}

struct MyPlugin;

impl HttpContext for MyPlugin {}

impl Context for MyPlugin {}

pub struct MyRootContext;

#[derive(serde::Deserialize, Debug)]
struct MyVmConfiguration {
    pub key: String,
}

impl RootContext for MyRootContext {
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyPlugin {}))
    }

    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        let vm_configuration = self.get_vm_configuration().unwrap();

        let parsed_vm_configuration =
            serde_json::from_slice::<MyVmConfiguration>(&vm_configuration).unwrap();

        debug!("vm_configuration: {:?}", parsed_vm_configuration);

        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for MyRootContext {}
