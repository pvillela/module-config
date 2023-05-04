use super::get_bar_bf;
use crate::fs::{foo_sfl, FooSflCfg, FooSflDeps, FooSflT, FOO_SFL_CFG, FOO_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::{init_option, RefreshMode};
use std::sync::Arc;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_sfl_raw(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    unsafe {
        init_option(&mut FOO_SFL_CFG, cfg);
        init_option(&mut FOO_SFL_DEPS, deps);
    }
    foo_sfl
}

pub fn get_foo_sfl(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooSflT {
    // A stereotype should initialize its dependencies.
    let bar_bf = get_bar_bf(app_cfg_src, refresh_mode.clone());
    let deps = FooSflDeps { bar_bf };
    get_foo_sfl_raw(
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
