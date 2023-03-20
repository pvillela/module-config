use super::{bar_a_bf_init_no_refresh, bar_a_bf_init_refreshable, bar_bf_init_refreshable};
use crate::fs::{bar_a_bf, FooASflDeps, FOO_A_SFL_CFG_DEF, FOO_A_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{arc_pin_async_fn, set_once_cell, CfgDef, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn foo_a_sfl_adapt_cfg_src(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    CfgDef::set_once_cell_with_cfg_adapter(
        &FOO_A_SFL_CFG_DEF,
        origin,
        foo_a_sfl_cfg_adapter,
        refresh_mode,
    );
}

pub fn foo_a_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, cache_ttl: Duration) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(app_cfg_src, cache_ttl);
    foo_a_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::Refreshable(cache_ttl));
    bar_bf_init_refreshable(app_cfg_src);
    let _ = set_once_cell(
        &FOO_A_SFL_DEPS,
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    );
}

pub fn foo_a_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_no_refresh(app_cfg_src);
    foo_a_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh);
    bar_bf_init_refreshable(app_cfg_src);
    let _ = set_once_cell(
        &FOO_A_SFL_DEPS,
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    );
}
