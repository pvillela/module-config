use super::{bar_bf, bar_bf_init_cached, bar_bf_init_refreshable};
use crate::fwk::{CfgDeps, RefreshMode};
use common::config::AppCfgInfo;
use std::{sync::OnceLock, time::Duration};

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub static FOO_SFL_CFG_DEPS: OnceLock<CfgDeps<FooSflCfgInfo, FooSflDeps>> = OnceLock::new();

pub fn foo_sfl() -> String {
    let (cfg, FooSflDeps { bar_bf }) = CfgDeps::get(&FOO_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
}

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> AppCfgInfo + 'static + Send + Sync,
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

pub fn foo_sfl_init_refreshable(app_cfg_src: fn() -> AppCfgInfo) {
    // A stereotype should initialize its dependencies.
    bar_bf_init_refreshable(app_cfg_src);
    foo_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(0)),
        FooSflDeps { bar_bf },
    );
}

pub fn foo_sfl_init_cached(app_cfg_src: fn() -> AppCfgInfo) {
    // A stereotype should initialize its dependencies.
    bar_bf_init_cached(app_cfg_src);
    foo_sfl_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh, FooSflDeps { bar_bf });
}
