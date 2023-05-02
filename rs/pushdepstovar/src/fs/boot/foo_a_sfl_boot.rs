use super::bar_a_bf_init;
use crate::fs::{bar_a_bf, FooASflCfg, FooASflDeps, FOO_A_SFL_CFG, FOO_A_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{arc_pin_async_fn, init_option, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_a_sfl_init(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init(origin, refresh_mode.clone());
    unsafe {
        init_option(
            &mut FOO_A_SFL_CFG,
            FooASflCfg::new_boxed_with_cfg_adapter(origin, foo_a_sfl_cfg_adapter, refresh_mode),
        );
        init_option(
            &mut FOO_A_SFL_DEPS,
            FooASflDeps {
                bar_a_bf: arc_pin_async_fn(bar_a_bf),
            },
        );
    }
}

pub fn foo_a_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_millis: u64) {
    foo_a_sfl_init(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(refresh_millis)),
    );
}

pub fn foo_a_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    foo_a_sfl_init(app_cfg_src, RefreshMode::NoRefresh);
}
