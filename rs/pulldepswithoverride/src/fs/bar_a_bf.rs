use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgOvd, CfgRefCellRc, RefreshMode};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

type BarABfCfg = CfgRefCellRc<BarABfCfgInfo>;

pub type BarABfCfgOvd = CfgOvd<BarABfCfgInfo>;

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = BAR_A_BF_CFG.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_A_BF_CFG: BarABfCfg =
    BarABfCfg::new_with_override(
        BAR_A_BF_CFG_OVERRIDE.get(),
        get_app_configuration,
        bar_a_bf_cfg_adapter,
        RefreshMode::NoRefresh,
    )
}

pub static BAR_A_BF_CFG_OVERRIDE: OnceCell<BarABfCfgOvd> = OnceCell::new();

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}
