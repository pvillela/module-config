use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgDepsArcSwapArc};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflT = ArcPinFn<FooAIn, FooAOut>;

pub type FooASflCfgDeps = CfgDepsArcSwapArc<FooASflCfgInfo, FooASflDeps>;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub fn foo_a_sfl_c(cfg_deps: FooASflCfgDeps) -> FooASflT {
    let d = cfg_deps.get_deps();
    let f = move |input: FooAIn| {
        let c = cfg_deps.get_cfg();
        let bar = &d.bar_a_bf;
        async move {
            let FooAIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = bar(0).await;
            let res = foo_core(a, b, bar_res);
            FooAOut { res }
        }
    };
    arc_pin_async_fn(f)
}
