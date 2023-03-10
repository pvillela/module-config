use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{CfgDepsInnerMut, RefreshMode};

type BarBfCfgInfo = common::fs_data::BarBfCfgInfo;

pub fn bar_bf() -> String {
    let (cfg, _) = BAR_BF_CFG_DEPS.with(|c| c.get());
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

thread_local! {
pub static BAR_BF_CFG_DEPS: CfgDepsInnerMut<BarBfCfgInfo, ()> = {
    CfgDepsInnerMut::new_with_cfg_adapter(
        get_app_configuration,
        bar_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        (),
    )
}
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
