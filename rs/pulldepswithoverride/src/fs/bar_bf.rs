use arc_swap::ArcSwap;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{InnerMut, RefreshMode};
use once_cell::sync::Lazy;

type BarBfCfgInfo = common::fs_data::BarBfCfgInfo;

pub fn bar_bf() -> String {
    let (cfg, _) = InnerMut::get_from_static(&BAR_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

pub static BAR_BF_CFG_DEPS: Lazy<ArcSwap<InnerMut<BarBfCfgInfo, ()>>> = Lazy::new(move || {
    ArcSwap::new(InnerMut::new_with_cfg_adapter(
        get_app_configuration,
        bar_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        (),
    ))
});

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
