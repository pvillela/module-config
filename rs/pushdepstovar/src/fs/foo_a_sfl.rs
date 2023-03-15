use common::{
    fs_data::{FooAIn, FooAOut},
    fs_util::foo_core,
    fwk::{ArcPinFn, CfgDepsArc},
};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub static FOO_A_SFL_CFG_DEPS: OnceCell<CfgDepsArc<FooASflCfgInfo, FooASflDeps>> = OnceCell::new();

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDepsArc::get_from_once_cell(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}
