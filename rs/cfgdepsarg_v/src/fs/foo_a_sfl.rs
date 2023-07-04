use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{ArcPinFn, CfgArcSwapArc};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflT = ArcPinFn<FooAIn, FooAOut>;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

// #[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub struct FooASflS {
    pub cfg: FooASflCfg,
    pub deps: FooASflDeps,
}

pub async fn foo_a_sfl_c(s: Arc<FooASflS>, input: FooAIn) -> FooAOut {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_a_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}
