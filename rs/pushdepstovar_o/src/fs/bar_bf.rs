use crate::fwk::{CfgDeps, RefreshMode};
use common::config::{get_app_configuration, AppCfgInfo};
use std::sync::{Arc, OnceLock};
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

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<BarBfCfgInfo> {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}

pub fn bar_bf_init_refreshable() {
    CfgDeps::set(
        &BAR_BF_CFG_DEPS,
        || bar_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::Refreshable(Duration::from_millis(0)),
        (),
    );
}

pub fn bar_bf_init_no_refresh() {
    CfgDeps::set(
        &BAR_BF_CFG_DEPS,
        || bar_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::NoRefresh,
        (),
    );
}
