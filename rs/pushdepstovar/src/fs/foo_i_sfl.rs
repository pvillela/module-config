use common::{fs_data::FooISflCfgInfo, fs_util::foo_core, fwk::get_from_once_cell};
use once_cell::sync::OnceCell;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

pub fn foo_i_sfl() -> String {
    let cfg = get_from_once_cell(&FOO_I_SFL_CFG);
    let FooISflDeps { bar_i_bf } = get_from_once_cell(&FOO_I_SFL_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

pub static FOO_I_SFL_DEPS: OnceCell<FooISflDeps> = OnceCell::new();

pub static FOO_I_SFL_CFG: OnceCell<FooISflCfgInfo> = OnceCell::new();
