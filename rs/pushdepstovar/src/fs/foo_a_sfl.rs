use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDeps, CfgRefCellRc, Pinfn},
    pin_async_fn,
};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub type FooASflT = Pinfn<FooAIn, FooAOut>;

pub struct FooASflDeps {
    pub bar_a_bf: Pinfn<u64, String>,
}

async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf } = FOO_A_SFL_CFG_DEPS.get_deps();

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = FOO_A_SFL_CFG_DEPS.get_cfg().get_cfg();

    let (a, b) = {
        let cfg = FOO_A_SFL_CFG_TL.with(|c| c.get_cfg());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

pub static FOO_A_SFL_CFG_DEPS: CfgDeps<FooASflCfg, FooASflDeps> = CfgDeps::new();

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_to_thread_local(FOO_A_SFL_CFG_DEPS.get_cfg());
}

pub fn get_foo_a_sfl_raw(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let _ = FOO_A_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_A_SFL_CFG_DEPS.set_deps_lenient(deps);
    pin_async_fn!(foo_a_sfl)
}
