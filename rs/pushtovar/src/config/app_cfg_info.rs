use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppCfgInfo {
    pub x: String,
    pub y: i32,
}

pub static APP_CONFIGURATION: Lazy<ArcSwap<AppCfgInfo>> =
    Lazy::new(|| ArcSwap::from_pointee(initial_app_configuration()));

// Simulates initial APP_CONFIGURATION
pub fn initial_app_configuration() -> AppCfgInfo {
    AppCfgInfo {
        x: "xxx".to_owned(),
        y: 42,
    }
}

// Simulates refresh of APP_CONFIGURATION
pub fn refresh_app_configuration() {
    APP_CONFIGURATION.store(Arc::new(AppCfgInfo {
        x: "yyy".to_owned(),
        y: 123,
    }));
}

pub fn get_app_configuration() -> Arc<AppCfgInfo> {
    APP_CONFIGURATION.load().clone()
}
