use super::bar_a_bf_init_refreshable;
use crate::config::AppCfgInfo;
use crate::fs::{bar_a_bf, FooASflCfgInfo, FooASflDeps, FOO_A_SFL_CFG_DEPS};
use crate::fwk::box_pin_async_fn;
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn foo_a_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: FooASflDeps,
) {
    CfgDeps::set_with_cfg_adapter(
        &FOO_A_SFL_CFG_DEPS,
        origin,
        foo_a_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn foo_a_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(app_cfg_src);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}

pub fn foo_a_sfl_init_cached(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(app_cfg_src);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Cached,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}
