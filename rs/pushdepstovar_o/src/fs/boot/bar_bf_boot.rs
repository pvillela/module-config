use crate::fs::{BarBfCfgInfo, BAR_BF_CFG_DEPS};
use crate::fwk::{CfgDeps, RefreshMode};
use common::config::{get_app_configuration, AppCfgInfo};
use std::sync::Arc;
use std::time::Duration;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<BarBfCfgInfo> {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
    .into()
}

pub fn bar_bf_init_refreshable() {
    CfgDeps::set(
        &BAR_BF_CFG_DEPS,
        || bar_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::Refreshable(Duration::from_millis(0)),
        (),
    );
}

pub fn bar_bf_init_no_refresh() {
    CfgDeps::set(
        &BAR_BF_CFG_DEPS,
        || bar_bf_cfg_adapter(&get_app_configuration()),
        RefreshMode::NoRefresh,
        (),
    );
}
