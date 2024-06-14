use crate::fwk::const_or_adapt_by_ref;
use common::config::AppCfgInfo;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub struct BarBfCfgSrc {
    pub get: Box<dyn Fn() -> Arc<BarBfCfgInfo> + Send + Sync>,
}

pub type BarBfT = Arc<dyn Fn() -> String + Send + Sync>;

pub fn bar_bf_c(cfg: BarBfCfgSrc) -> BarBfT {
    Arc::new(move || {
        let cfg = (cfg.get)();
        let u = cfg.u + 1;
        let v = cfg.v.clone() + "-bar";
        format!("barBf(): u={}, v={}", u, v)
    })
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub static BAR_BF_CFG_INFO_OVERRIDE: OnceLock<BarBfCfgInfo> = OnceLock::new();

pub fn bar_bf_boot(app_cfg: fn() -> AppCfgInfo) -> BarBfT {
    let get = const_or_adapt_by_ref(BAR_BF_CFG_INFO_OVERRIDE.get(), app_cfg, bar_bf_cfg_adapter);
    let bar_bf_cfg_src = BarBfCfgSrc { get };
    bar_bf_c(bar_bf_cfg_src)
}
