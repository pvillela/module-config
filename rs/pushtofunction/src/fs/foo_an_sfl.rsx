use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn, arc_pin_async_fn_l, ArcPinFn, ArcPinFnL, CfgDepsArcSwapArcNc};
use std::time::Duration;
use tokio::time::sleep;

pub type FooAnSflT<'a> = ArcPinFnL<'a, FooAnIn, FooAnOut>;
type FooAnIn = FooAIn;
type FooAnOut = FooAOut;
type FooAnSflCfgInfo = FooASflCfgInfo;

pub type FooAnSflCfgDeps = CfgDepsArcSwapArcNc<FooAnSflCfgInfo, FooAnSflDeps>;

// #[derive(Clone)]
pub struct FooAnSflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

// impl std::fmt::Debug for FooAnSflDeps {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("<FooAnSflDeps>")
//     }
// }

pub fn foo_an_sfl_c<'a>(cfg_deps: &'a FooAnSflCfgDeps) -> FooAnSflT {
    let f = move |input: FooAnIn| {
        let c = cfg_deps.get_cfg();
        async move {
            let d = &cfg_deps.get_deps();
            let FooAnIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = (d.bar_a_bf)(0).await;
            let res = foo_core(a, b, bar_res);
            FooAnOut { res }
        }
    };
    arc_pin_async_fn_l(f)
}
