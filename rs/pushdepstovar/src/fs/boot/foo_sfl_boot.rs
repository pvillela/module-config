use super::{bar_bf_init_cached, bar_bf_init_refreshable};
use crate::fs::{bar_bf, FooSflDeps, FOO_SFL_CFG_DEF, FOO_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::{CfgDef, RefreshMode};
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
) {
    CfgDef::set_once_cell_with_cfg_adapter(
        &FOO_SFL_CFG_DEF,
        origin,
        foo_sfl_cfg_adapter,
        refresh_mode,
    );
}

pub fn foo_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    foo_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(0)),
    );
    bar_bf_init_refreshable(app_cfg_src);
    FOO_SFL_DEPS.set(FooSflDeps { bar_bf });
}

pub fn foo_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    foo_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh);
    // A stereotype should initialize its dependencies.
    bar_bf_init_cached(app_cfg_src);
    FOO_SFL_DEPS.set(FooSflDeps { bar_bf });
}
