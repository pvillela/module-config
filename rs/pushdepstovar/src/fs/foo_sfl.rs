use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, get_from_once_lock, set_once_lock, CfgArcSwapArc, CfgRefCellRc},
};
use std::sync::OnceLock;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflT = fn() -> String;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

fn foo_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = get_cfg().get_cfg();

    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

static FOO_SFL_CFG: OnceLock<FooSflCfg> = OnceLock::new();

fn get_cfg() -> &'static FooSflCfg {
    get_from_once_lock(&FOO_SFL_CFG)
}

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_to_thread_local(get_cfg());
}

static FOO_SFL_DEPS: OnceLock<FooSflDeps> = OnceLock::new();

fn get_deps() -> &'static FooSflDeps {
    get_from_once_lock(&FOO_SFL_DEPS)
}

pub fn get_foo_sfl_raw(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let _ = set_once_lock(&FOO_SFL_CFG, cfg);
    let _ = set_once_lock(&FOO_SFL_DEPS, deps);
    foo_sfl
}
