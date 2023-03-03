use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[derive(Debug, Clone)]
pub struct AppCfgInfo {
    pub x: String,
    pub y: i32,
    pub z: bool,
}

static APP_CONFIGURATION: Lazy<ArcSwap<AppCfgInfo>> =
    Lazy::new(|| ArcSwap::from_pointee(initial_app_configuration()));

// Simulates initial APP_CONFIGURATION
fn initial_app_configuration() -> AppCfgInfo {
    AppCfgInfo {
        x: "initial".to_owned(),
        y: 42,
        z: false,
    }
}

static REFRESH_COUNT: AtomicU32 = AtomicU32::new(0);

// Simulates refresh of APP_CONFIGURATION
pub fn refresh_app_configuration() {
    let count = REFRESH_COUNT.fetch_add(1, Ordering::Relaxed);
    APP_CONFIGURATION.store(Arc::new(AppCfgInfo {
        x: format!("refreshed-{}", count),
        y: 1042,
        z: true,
    }));
}

pub fn get_app_configuration() -> Arc<AppCfgInfo> {
    // println!("get_app_configuration has been called");
    APP_CONFIGURATION.load().clone()
}
