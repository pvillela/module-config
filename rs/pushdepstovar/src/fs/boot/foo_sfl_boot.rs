use super::{bar_bf_init_cached, bar_bf_init_refreshable};
use crate::fs::{bar_bf, FooSflCfgInfo, FooSflDeps, FOO_SFL_CFG_DEPS};
use common::config::AppCfgInfo;
use common::fwk::{CfgDepsArc, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: FooSflDeps,
) {
    CfgDepsArc::set_with_cfg_adapter(
        &FOO_SFL_CFG_DEPS,
        origin,
        foo_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn foo_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_bf_init_refreshable(app_cfg_src);
    foo_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(0)),
        FooSflDeps { bar_bf },
    );
}

pub fn foo_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_bf_init_cached(app_cfg_src);
    foo_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh, FooSflDeps { bar_bf });
}
