use crate::config::{get_app_config_info, AppCfgInfo};
use crate::fs::{BarABfCfgInfo, BAR_A_BF_CFG_DEPS};
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<BarABfCfgInfo> {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}

pub fn bar_a_bf_init_refreshable(cache_ttl: Duration) {
    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        || bar_a_bf_cfg_adapter(&get_app_config_info()),
        RefreshMode::Refreshable(cache_ttl),
        (),
    );
}

pub fn bar_a_bf_init_no_refresh() {
    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        || bar_a_bf_cfg_adapter(&get_app_config_info()),
        RefreshMode::NoRefresh,
        (),
    );
}
