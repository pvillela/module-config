use crate::fs::bar_i_bf::get_bar_i_bf_with_app_cfg;
use common::config::AppCfgInfo;
use common::{fs_data::FooISflCfgInfo, fs_util::foo_core, fwk::CfgDepsS};
use std::rc::Rc;

pub type FooISflT = fn() -> String;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

fn foo_i_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = FOO_I_SFL_CFG_DEPS.get_cfg();

    let cfg = FOO_I_SFL_CFG_TL.with(|c| c.clone());
    let FooISflDeps { bar_i_bf } = FOO_I_SFL_CFG_DEPS.get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

static FOO_I_SFL_CFG_DEPS: CfgDepsS<FooISflCfgInfo, FooISflDeps> = CfgDepsS::new();

thread_local! {
    pub static FOO_I_SFL_CFG_TL: Rc<FooISflCfgInfo> = Rc::new(FOO_I_SFL_CFG_DEPS.get_cfg().clone());
}

pub fn get_foo_i_sfl_raw(cfg: FooISflCfgInfo, deps: FooISflDeps) -> FooISflT {
    let _ = FOO_I_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_I_SFL_CFG_DEPS.set_deps_lenient(deps);
    foo_i_sfl
}

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_i_sfl_with_app_cfg(app_cfg: &AppCfgInfo) -> FooISflT {
    // A stereotype should initialize its dependencies.
    let bar_i_bf = get_bar_i_bf_with_app_cfg(app_cfg);
    let deps = FooISflDeps { bar_i_bf };
    get_foo_i_sfl_raw(foo_i_sfl_cfg_adapter(app_cfg), deps)
}
