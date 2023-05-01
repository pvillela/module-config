use common::{fs_data::FooISflCfgInfo, fs_util::foo_core, fwk::get_initialized_option};

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

pub fn foo_i_sfl() -> String {
    let cfg = get_my_cfg();
    let FooISflDeps { bar_i_bf } = get_my_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

pub static mut FOO_I_SFL_DEPS: Option<FooISflDeps> = None;

pub static mut FOO_I_SFL_CFG: Option<FooISflCfgInfo> = None;

fn get_my_cfg() -> &'static FooISflCfgInfo {
    unsafe { get_initialized_option(&FOO_I_SFL_CFG) }
}

fn get_my_deps() -> &'static FooISflDeps {
    unsafe { get_initialized_option(&FOO_I_SFL_DEPS) }
}
