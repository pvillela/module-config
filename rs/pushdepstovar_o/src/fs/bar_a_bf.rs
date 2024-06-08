use crate::fwk::{CfgDeps, RefreshMode};
use common::config::{get_app_configuration, AppCfgInfo};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BarABfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_A_BF_CFG_DEPS: OnceLock<CfgDeps<BarABfCfgInfo, ()>> = OnceLock::new();

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, _) = CfgDeps::get(&BAR_A_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<BarABfCfgInfo> {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}

pub fn bar_a_bf_init_refreshable(cache_ttl: Duration) {
    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        || bar_a_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::Refreshable(cache_ttl),
        (),
    );
}

pub fn bar_a_bf_init_no_refresh() {
    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        || bar_a_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::NoRefresh,
        (),
    );
}
