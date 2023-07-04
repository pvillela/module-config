use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDepsS, CfgRefCellRc},
};

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
