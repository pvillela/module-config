use common::fs_data::{FooAwIn, FooAwOut, FooAwSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn_web, ArcPinFnWeb, CfgDepsRefCellId};
use std::time::Duration;
use tokio::time::sleep;

pub type FooAwSflT = ArcPinFnWeb<FooAwIn, FooAwOut>;

pub type FooAwSflCfgDeps = CfgDepsRefCellId<FooAwSflCfgInfo, FooAwSflDeps>;

#[derive(Clone)]
pub struct FooAwSflDeps {
    pub bar_aw_bf: ArcPinFnWeb<u64, String>,
}

pub fn foo_aw_sfl_c(cfg_deps: FooAwSflCfgDeps) -> FooAwSflT {
    let f = move |input: FooAwIn| {
        let (c, d) = cfg_deps.get_cfg_deps();
        async move {
            let FooAwIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = (d.bar_aw_bf)(0).await;
            let res = foo_core(a, b, bar_res);
            FooAwOut { res }
        }
    };
    arc_pin_async_fn_web(f)
}
