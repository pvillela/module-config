use super::{bar_a_bf_init_no_refresh, bar_a_bf_init_refreshable};
use crate::config::{get_app_config_info, AppCfgInfo};
use crate::fs::{bar_a_bf, FooASflCfgInfo, FooASflDeps, FOO_A_SFL_CFG_DEPS};
use crate::fwk::box_pin_async_fn;
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<FooASflCfgInfo> {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
    .into()
}

pub fn foo_a_sfl_init_refreshable(cache_ttl: Duration) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(cache_ttl);
    CfgDeps::set(
        &FOO_A_SFL_CFG_DEPS,
        || foo_a_sfl_cfg_adapter(&get_app_config_info()),
        RefreshMode::Refreshable(cache_ttl),
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}

pub fn foo_a_sfl_init_no_refresh() {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_no_refresh();
    CfgDeps::set(
        &FOO_A_SFL_CFG_DEPS,
        || foo_a_sfl_cfg_adapter(&get_app_config_info()),
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}
