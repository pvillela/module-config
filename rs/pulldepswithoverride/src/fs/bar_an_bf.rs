use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDepsOvr, CfgDepsRefCellRcNc, RefreshMode};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

type BarAnBfCfgDeps = CfgDepsRefCellRcNc<BarABfCfgInfo, ()>;
type BarAnBfCfgInfo = BarABfCfgInfo;

pub type BarAnBfCfgDepsOvr = CfgDepsOvr<BarABfCfgInfo, ()>;

pub async fn bar_an_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = BAR_AN_BF_CFG_DEPS.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_AN_BF_CFG_DEPS: BarAnBfCfgDeps =
    BarAnBfCfgDeps::new_with_override(
        BAR_AN_BF_CFG_DEPS_OVERRIDE.get(),
        get_app_configuration,
        bar_an_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        (),
    )
}

pub static BAR_AN_BF_CFG_DEPS_OVERRIDE: OnceCell<BarAnBfCfgDepsOvr> = OnceCell::new();

fn bar_an_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAnBfCfgInfo {
    BarAnBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}
