use std::time::Duration;

use super::bar_a_bf::BarABfD;
use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{CfgArcSwapArc, Dep1a},
};
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf_d: BarABfD,
}

pub struct FooASflS {
    pub cfg: FooASflCfg,
    pub deps: FooASflDeps,
}

pub type FooASflD = Dep1a<FooASflS, FooAOut, FooAIn>;

pub async fn foo_a_sfl_c(s: &FooASflS, input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf_d } = &s.deps;
    let cfg = s.cfg.get_cfg();
    let a = cfg.a.clone();
    let b = cfg.b;

    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar_a_bf_d.run(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}
