use crate::config::AppCfgInfo;
use crate::fs::{bar_bf, FooSflCfgInfo, FooSflDeps, FOO_SFL_CFG_DEPS};
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: FooSflDeps,
) {
    CfgDeps::set_with_cfg_adapter(
        &FOO_SFL_CFG_DEPS,
        origin,
        foo_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn foo_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    foo_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::Refreshable, FooSflDeps { bar_bf });
}

pub fn foo_sfl_init_cached(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    foo_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::Cached, FooSflDeps { bar_bf });
}
