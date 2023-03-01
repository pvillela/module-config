use crate::config::{get_app_configuration, AppCfgInfo};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use pdwo_arch::fwk::{CfgDeps, RefreshMode};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BarABfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, _) = CfgDeps::get(&BAR_A_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

pub static BAR_A_BF_CFG_DEPS: Lazy<ArcSwap<CfgDeps<BarABfCfgInfo, ()>>> = Lazy::new(move || {
    ArcSwap::new(CfgDeps::new_with_cfg_adapter(
        get_app_configuration,
        bar_a_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        (),
    ))
});

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
