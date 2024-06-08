use super::{bar_bf, bar_bf_init_no_refresh, bar_bf_init_refreshable};
use crate::fwk::{CfgDeps, RefreshMode};
use common::config::{get_app_configuration, AppCfgInfo};
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Clone)]
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

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> Arc<FooSflCfgInfo> {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
    .into()
}

pub fn foo_sfl_init_refreshable() {
    // A stereotype should initialize its dependencies.
    bar_bf_init_refreshable();
    CfgDeps::set(
        &FOO_SFL_CFG_DEPS,
        || foo_sfl_cfg_adapter(&get_app_configuration()),
        RefreshMode::Refreshable(Duration::from_millis(0)),
        FooSflDeps { bar_bf },
    );
}

pub fn foo_sfl_init_no_refresh() {
    // A stereotype should initialize its dependencies.
    bar_bf_init_no_refresh();
    CfgDeps::set(
        &FOO_SFL_CFG_DEPS,
        || foo_sfl_cfg_adapter(&get_app_configuration()),
        RefreshMode::NoRefresh,
        FooSflDeps { bar_bf },
    );
}
