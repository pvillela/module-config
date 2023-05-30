use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{cfg_global_to_thread_local, get_initialized_option, CfgArcSwapArc, CfgRefCellRc},
};

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflT = fn() -> String;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub(in crate::fs) fn foo_sfl() -> String {
    // This is to demonstrate calling get_my_cfg() as an alternative to using the thread-local..
    let _ = get_my_cfg().get_cfg();

    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = get_my_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = unsafe {cfg_global_to_thread_local(&FOO_SFL_CFG)};
}

pub static mut FOO_SFL_DEPS: Option<FooSflDeps> = None;

pub static mut FOO_SFL_CFG: Option<FooSflCfg> = None;

fn get_my_cfg() -> &'static FooSflCfg {
    unsafe { get_initialized_option(&FOO_SFL_CFG) }
}

fn get_my_deps() -> &'static FooSflDeps {
    unsafe { get_initialized_option(&FOO_SFL_DEPS) }
}
