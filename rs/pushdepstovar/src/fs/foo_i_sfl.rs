use common::{
    fs_data::FooISflCfgInfo,
    fs_util::foo_core,
    fwk::{get_from_once_cell, set_once_cell},
};
use std::sync::OnceLock;

pub type FooISflT = fn() -> String;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

fn foo_i_sfl() -> String {
    let cfg = get_from_once_cell(&FOO_I_SFL_CFG);
    let FooISflDeps { bar_i_bf } = get_from_once_cell(&FOO_I_SFL_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

static FOO_I_SFL_DEPS: OnceLock<FooISflDeps> = OnceLock::new();

static FOO_I_SFL_CFG: OnceLock<FooISflCfgInfo> = OnceLock::new();

pub fn get_foo_i_sfl_raw(cfg: FooISflCfgInfo, deps: FooISflDeps) -> FooISflT {
    let _ = set_once_cell(&FOO_I_SFL_CFG, cfg);
    let _ = set_once_cell(&FOO_I_SFL_DEPS, deps);
    foo_i_sfl
}
