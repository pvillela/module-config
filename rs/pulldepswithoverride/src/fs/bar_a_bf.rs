use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{CfgDepsInnerMutArc, RefreshMode};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::sleep;

type BarBfCfgInfo = common::fs_data::BarBfCfgInfo;

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, _) = BAR_A_BF_CFG_DEPS.get();
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

pub static BAR_A_BF_CFG_DEPS: Lazy<CfgDepsInnerMutArc<BarBfCfgInfo, ()>> = Lazy::new(move || {
    CfgDepsInnerMutArc::new_with_cfg_adapter(
        get_app_configuration,
        bar_a_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        // RefreshMode::Refreshable(Duration::from_millis(60)),
        (),
    )
});

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
