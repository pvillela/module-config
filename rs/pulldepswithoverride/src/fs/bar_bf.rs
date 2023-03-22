use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgOvd, CfgRefCellRc, RefreshMode};
use once_cell::sync::OnceCell;

type BarBfCfg = CfgRefCellRc<BarBfCfgInfo>;

pub type BarBfCfgOvd = CfgOvd<BarBfCfgInfo>;

pub fn bar_bf() -> String {
    let cfg = BAR_BF_CFG.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_BF_CFG: BarBfCfg =
    BarBfCfg::new_boxed_with_cfg_adapter_and_override(BAR_BF_CFG_OVERRIDE.get(),
        get_app_configuration,
        bar_bf_cfg_adapter,
        RefreshMode::NoRefresh,
)
}

pub static BAR_BF_CFG_OVERRIDE: OnceCell<BarBfCfgOvd> = OnceCell::new();

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
