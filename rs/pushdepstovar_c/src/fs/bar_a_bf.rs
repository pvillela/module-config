use crate::fwk::{CfgDeps, RefreshMode};
use common::config::AppCfgInfo;
use std::sync::OnceLock;
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

fn bar_abf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn bar_abf_adapt_cfg_src(
    origin: impl Fn() -> AppCfgInfo + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: (),
) {
    CfgDeps::set_with_cfg_adapter(
        &BAR_A_BF_CFG_DEPS,
        origin,
        bar_abf_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn bar_a_bf_init_refreshable(app_cfg_src: fn() -> AppCfgInfo, cache_ttl: Duration) {
    bar_abf_adapt_cfg_src(app_cfg_src, RefreshMode::Refreshable(cache_ttl), ());
}

pub fn bar_a_bf_init_no_refresh(app_cfg_src: fn() -> AppCfgInfo) {
    bar_abf_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh, ());
}
