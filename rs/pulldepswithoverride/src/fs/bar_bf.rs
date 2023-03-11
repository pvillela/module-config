use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{CfgDepsDefault, RefreshMode};

type BarBfCfgInfo = common::fs_data::BarBfCfgInfo;

pub fn bar_bf() -> String {
    let (cfg, _) = BAR_BF_CFG_DEPS.with(|c| c.get());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_BF_CFG_DEPS: CfgDepsDefault<BarBfCfgInfo, ()> = {
    CfgDepsDefault::new_with_cfg_adapter(
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
