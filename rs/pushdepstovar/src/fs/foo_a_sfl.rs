use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{
        cfg_once_cell_to_thread_local, get_from_once_cell, set_once_cell, CfgArcSwapArc,
        CfgRefCellRc, Pinfn,
    },
    pin_async_fn,
};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub type FooASflT = Pinfn<FooAIn, FooAOut>;

pub struct FooASflDeps {
    pub bar_a_bf: Pinfn<u64, String>,
}

async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf } = get_from_once_cell(&FOO_A_SFL_DEPS);
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instea of thread-local.
    let _ = get_from_once_cell(&FOO_A_SFL_CFG).get_cfg();

    let (a, b) = {
        let cfg = FOO_A_SFL_CFG_TL.with(|c| c.get_cfg());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_once_cell_to_thread_local(&FOO_A_SFL_CFG);
}

static FOO_A_SFL_DEPS: OnceLock<FooASflDeps> = OnceLock::new();

static FOO_A_SFL_CFG: OnceLock<FooASflCfg> = OnceLock::new();

pub fn get_foo_a_sfl_raw(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let _ = set_once_cell(&FOO_A_SFL_CFG, cfg);
    let _ = set_once_cell(&FOO_A_SFL_DEPS, deps);
    pin_async_fn!(foo_a_sfl)
}
