use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{get_from_once_cell, ArcPinFn, CfgDef, CfgRefCellRc},
};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgRefCellRc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = FOO_A_SFL_CFG.with(|c| c.get_cfg());
    let FooASflDeps { bar_a_bf } = get_from_once_cell(&FOO_A_SFL_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

thread_local! {
static FOO_A_SFL_CFG: FooASflCfg =
    FooASflCfg::new_from_def(
        FOO_A_SFL_CFG_DEF.get(),
    )
}

pub static FOO_A_SFL_CFG_DEF: OnceCell<CfgDef<FooASflCfgInfo>> = OnceCell::new();
pub static FOO_A_SFL_DEPS: OnceCell<FooASflDeps> = OnceCell::new();
