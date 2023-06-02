use super::get_bar_a_bf_with_app_cfg;
use crate::fs::{get_foo_a_sfl_raw, FooASflCfg, FooASflDeps, FooASflT};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_a_sfl_wtih_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> FooASflT {
    // A stereotype should initialize its dependencies.
    let bar_a_bf = get_bar_a_bf_with_app_cfg(app_cfg_src, refresh_mode.clone());
    let deps = FooASflDeps { bar_a_bf };
    get_foo_a_sfl_raw(
        FooASflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_a_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
