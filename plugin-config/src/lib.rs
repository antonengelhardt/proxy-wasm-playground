// proxy-wasm
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use log::debug;

// This is the initial entry point of the plugin.
proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);

    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyRootContext{}) });
}}

pub struct MyRootContext;

#[derive(serde::Deserialize)]
#[derive(Debug)]
struct MyPluginConfiguration {
    pub key: String,
}

impl RootContext for MyRootContext {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        let plugin_configuration = self.get_plugin_configuration().unwrap();

        let parsed_plugin_configuration = serde_json::from_slice::<MyPluginConfiguration>(&plugin_configuration).unwrap();

        debug!("plugin_configuration: {:?}", parsed_plugin_configuration);

        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for MyRootContext {}
