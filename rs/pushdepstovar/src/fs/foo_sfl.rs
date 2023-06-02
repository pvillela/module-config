use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{
        cfg_once_cell_to_thread_local, get_from_once_cell, set_once_cell, CfgArcSwapArc,
        CfgRefCellRc,
    },
};
use once_cell::sync::OnceCell;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflT = fn() -> String;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

fn foo_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _ = get_from_once_cell(&FOO_SFL_CFG).get_cfg();

    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = get_from_once_cell(&FOO_SFL_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_once_cell_to_thread_local(&FOO_SFL_CFG);
}

pub static FOO_SFL_DEPS: OnceCell<FooSflDeps> = OnceCell::new();

pub static FOO_SFL_CFG: OnceCell<FooSflCfg> = OnceCell::new();

pub fn get_foo_sfl_raw(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let _ = set_once_cell(&FOO_SFL_CFG, cfg);
    let _ = set_once_cell(&FOO_SFL_DEPS, deps);
    foo_sfl
}
