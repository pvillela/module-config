use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{box_pin_async_fn, BoxPinFn, CfgDepsArcSwapArc};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflT = BoxPinFn<FooAIn, FooAOut>;

pub type FooASflCfgDeps = CfgDepsArcSwapArc<FooASflCfgInfo, FooASflDeps>;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

// impl std::fmt::Debug for FooASflDeps {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("<FooASflDeps>")
//     }
// }

#[derive(Clone, Debug)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_a_sfl_c(cfg_deps: &FooASflCfgDeps) -> FooASflT {
    let cfg_deps = cfg_deps.clone();
    let f = move |input: FooAIn| {
        let (c, d) = cfg_deps.get();
        async move {
            let FooAIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = (d.bar_a_bf)(0).await;
            let res = foo_core(a, b, bar_res);
            FooAOut { res }
        }
    };
    box_pin_async_fn(f)
}
