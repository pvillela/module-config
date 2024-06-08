use crate::fs::bar_bf::get_bar_bf_with_app_cfg;
use common::config::AppCfgInfo;
use common::fwk::RefreshMode;
use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDepsS, CfgRefCellRc},
};
use std::sync::Arc;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflT = fn() -> String;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

fn foo_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = FOO_SFL_CFG_DEPS.get_cfg().get_cfg();

    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = FOO_SFL_CFG_DEPS.get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

static FOO_SFL_CFG_DEPS: CfgDepsS<FooSflCfg, FooSflDeps> = CfgDepsS::new();

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_to_thread_local(FOO_SFL_CFG_DEPS.get_cfg());
}

pub fn get_foo_sfl_raw(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let _ = FOO_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_SFL_CFG_DEPS.set_deps_lenient(deps);
    foo_sfl
}

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_sfl_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> FooSflT {
    // A stereotype should initialize its dependencies.
    let bar_bf = get_bar_bf_with_app_cfg(app_cfg_src, refresh_mode.clone());
    let deps = FooSflDeps { bar_bf };
    get_foo_sfl_raw(
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
