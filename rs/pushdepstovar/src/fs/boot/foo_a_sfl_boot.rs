use super::{bar_a_bf_init_no_refresh, bar_a_bf_init_refreshable};
use crate::fs::{bar_a_bf, FooASflCfgInfo, FooASflDeps, FOO_A_SFL_CFG_DEPS};
use common::config::AppCfgInfo;
use common::fwk::{box_pin_async_fn, CfgDepsArc, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

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
    CfgDepsArc::set_with_cfg_adapter(
        &FOO_A_SFL_CFG_DEPS,
        origin,
        foo_a_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn foo_a_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, cache_ttl: Duration) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(app_cfg_src, cache_ttl);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(cache_ttl),
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}

pub fn foo_a_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_no_refresh(app_cfg_src);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}
