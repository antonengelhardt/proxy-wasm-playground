// proxy-wasm
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use std::time::Duration;

use log::debug;

// This is the initial entry point of the plugin.
proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);

    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyRootContext{}) });
}}

pub struct MyRootContext;

impl RootContext for MyRootContext {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        self.set_tick_period(Duration::from_millis(5000));

        true
    }

    fn on_tick(&mut self) {
        debug!("on_tick");
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for MyRootContext {}
