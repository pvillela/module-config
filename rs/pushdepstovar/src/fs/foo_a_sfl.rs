use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{cfg_global_to_thread_local, get_initialized_option, CfgArcSwapArc, CfgRefCellRc, Pinfn},
};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub type FooASflT = Pinfn<FooAIn, FooAOut>;

pub struct FooASflDeps {
    pub bar_a_bf: Pinfn<u64, String>,
}

pub(in crate::fs) async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf } = get_my_deps();
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate calling get_my_cfg() as an alternative to using the thread-local..
    let _ = get_my_cfg().get_cfg();

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
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = unsafe {cfg_global_to_thread_local(&FOO_A_SFL_CFG)};
}

pub static mut FOO_A_SFL_DEPS: Option<FooASflDeps> = None;

pub static mut FOO_A_SFL_CFG: Option<FooASflCfg> = None;

fn get_my_cfg() -> &'static FooASflCfg {
    unsafe { get_initialized_option(&FOO_A_SFL_CFG) }
}

fn get_my_deps() -> &'static FooASflDeps {
    unsafe { get_initialized_option(&FOO_A_SFL_DEPS) }
}
