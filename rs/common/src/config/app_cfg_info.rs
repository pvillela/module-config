use crate::fwk::{Db, DbCfg};
use arc_swap::{ArcSwap, ArcSwapAny};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, OnceLock,
};

#[derive(Debug, Clone)]
pub struct AppCfgInfo {
    pub x: String,
    pub y: i32,
    pub z: bool,
}

static APP_CONFIGURATION: OnceLock<ArcSwap<AppCfgInfo>> = OnceLock::new();

static REFRESH_COUNT: AtomicU32 = AtomicU32::new(0);

// Produce simulated initial APP_CONFIGURATION
fn initial_app_configuration() -> AppCfgInfo {
    AppCfgInfo {
        x: "initial".to_owned(),
        y: 42,
        z: false,
    }
}

fn get_app_config_arcswap() -> &'static ArcSwapAny<Arc<AppCfgInfo>> {
    APP_CONFIGURATION.get_or_init(|| ArcSwap::from_pointee(initial_app_configuration()))
}

// Simulates initialization of APP_CONFIGURATION
pub fn initialize_app_configuration() {
    REFRESH_COUNT.store(0, Ordering::Relaxed);
    let cfg_as = get_app_config_arcswap();
    cfg_as.store(Arc::new(initial_app_configuration()));
}

// Simulates refresh of APP_CONFIGURATION
pub fn refresh_app_configuration() {
    let count = REFRESH_COUNT.fetch_add(1, Ordering::Relaxed);
    let cfg_as = get_app_config_arcswap();
    cfg_as.store(Arc::new(AppCfgInfo {
        x: format!("refreshed-{}", count),
        y: 1042,
        z: true,
    }));
}

pub fn get_app_configuration() -> Arc<AppCfgInfo> {
    // println!("get_app_configuration has been called");
    let cfg_as = get_app_config_arcswap();
    cfg_as.load().clone()
}

impl DbCfg for AppCfgInfo {
    fn get_db(&self) -> &'static Db {
        // TODO: implement this properly
        static APP_CFG_INFO_DB: OnceLock<Db> = OnceLock::new();
        APP_CFG_INFO_DB.get_or_init(|| Db)
    }
}
