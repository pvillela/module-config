use super::{bar_bf_init_no_refresh, bar_bf_init_refreshable};
use crate::config::{get_app_config_info, AppCfgInfo};
use crate::fs::{bar_bf, FooSflCfgInfo, FooSflDeps, FOO_SFL_CFG_DEPS};
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<FooSflCfgInfo> {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
    .into()
}

pub fn foo_sfl_init_refreshable() {
    // A stereotype should initialize its dependencies.
    bar_bf_init_refreshable();
    CfgDeps::set(
        &FOO_SFL_CFG_DEPS,
        || foo_sfl_cfg_adapter(&get_app_config_info()),
        RefreshMode::Refreshable(Duration::from_millis(0)),
        FooSflDeps { bar_bf },
    );
}

pub fn foo_sfl_init_no_refresh() {
    // A stereotype should initialize its dependencies.
    bar_bf_init_no_refresh();
    CfgDeps::set(
        &FOO_SFL_CFG_DEPS,
        || foo_sfl_cfg_adapter(&get_app_config_info()),
        RefreshMode::NoRefresh,
        FooSflDeps { bar_bf },
    );
}
