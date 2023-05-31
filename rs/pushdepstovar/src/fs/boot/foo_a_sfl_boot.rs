use crate::fs::{foo_a_sfl, FooASflCfg, FooASflDeps, FooASflT, FOO_A_SFL_CFG, FOO_A_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{set_once_cell, RefreshMode};
use common::pin_async_fn;
use std::sync::Arc;

use super::get_bar_a_bf;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_a_sfl_raw(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let _ = set_once_cell(&FOO_A_SFL_CFG, cfg);
    let _ = set_once_cell(&FOO_A_SFL_DEPS, deps);
    pin_async_fn!(foo_a_sfl)
}

pub fn get_foo_a_sfl(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooASflT {
    // A stereotype should initialize its dependencies.
    let bar_a_bf = get_bar_a_bf(app_cfg_src, refresh_mode.clone());
    let deps = FooASflDeps { bar_a_bf };
    get_foo_a_sfl_raw(
        FooASflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_a_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
