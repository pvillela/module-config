use super::BarABfT;
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{CfgArcSwapArc, CfgDeps, PinFn};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAdSflT = PinFn<FooAIn, FooAOut>;

pub type FooAdSflCfg = CfgArcSwapArc<FooASflCfgInfo>;

// #[derive(Clone)]
pub struct FooAdSflDeps {
    pub bar_a_bf: Box<BarABfT>,
}

pub type FooAdSflS = CfgDeps<FooAdSflCfg, FooAdSflDeps>;

pub async fn foo_ad_sfl_c(
    s: &(dyn Deref<Target = FooAdSflS> + Send + Sync),
    input: FooAIn,
) -> FooAOut {
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
