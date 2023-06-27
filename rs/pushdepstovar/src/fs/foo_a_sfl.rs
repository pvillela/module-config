use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{
        cfg_to_thread_local, get_from_once_lock, set_once_lock, CfgArcSwapArc, CfgRefCellRc, Pinfn,
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
    let FooASflDeps { bar_a_bf } = get_deps();
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = get_cfg().get_cfg();

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

static FOO_A_SFL_CFG: OnceLock<FooASflCfg> = OnceLock::new();

fn get_cfg() -> &'static FooASflCfg {
    get_from_once_lock(&FOO_A_SFL_CFG)
}

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_to_thread_local(get_cfg());
}

static FOO_A_SFL_DEPS: OnceLock<FooASflDeps> = OnceLock::new();

fn get_deps() -> &'static FooASflDeps {
    get_from_once_lock(&FOO_A_SFL_DEPS)
}

pub fn get_foo_a_sfl_raw(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let _ = set_once_lock(&FOO_A_SFL_CFG, cfg);
    let _ = set_once_lock(&FOO_A_SFL_DEPS, deps);
    pin_async_fn!(foo_a_sfl)
}
