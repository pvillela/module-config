use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDepsDefault, CfgDepsOvr, RefreshMode};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

type BarABfCfgDeps = CfgDepsDefault<BarABfCfgInfo, ()>;

pub type BarABfCfgDepsOvr = CfgDepsOvr<AppCfgInfo, BarABfCfgInfo, ()>;

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, _) = BAR_A_BF_CFG_DEPS.with(|c| c.get_cfg_deps());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_A_BF_CFG_DEPS: BarABfCfgDeps =
    BarABfCfgDeps::new_with_override(
        &BAR_A_BF_CFG_DEPS_OVERRIDE,
        get_app_configuration,
        bar_a_bf_cfg_adapter,
        RefreshMode::NoRefresh,
        (),
    );
}

pub static BAR_A_BF_CFG_DEPS_OVERRIDE: OnceCell<BarABfCfgDepsOvr> = OnceCell::new();

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}
