use crate::fwk::{CfgDeps, RefreshMode};
use common::config::AppCfgInfo;
use std::sync::OnceLock;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_DEPS: OnceLock<CfgDeps<BarBfCfgInfo, ()>> = OnceLock::new();

pub fn bar_bf() -> String {
    let (cfg, _) = CfgDeps::get(&BAR_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn bar_bf_adapt_cfg_src(
    origin: impl Fn() -> AppCfgInfo + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: (),
) {
    CfgDeps::set_with_cfg_adapter(
        &BAR_BF_CFG_DEPS,
        origin,
        bar_bf_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn bar_bf_init_refreshable(app_cfg_src: fn() -> AppCfgInfo) {
    bar_bf_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(0)),
        (),
    );
}

pub fn bar_bf_init_cached(app_cfg_src: fn() -> AppCfgInfo) {
    bar_bf_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh, ());
}
