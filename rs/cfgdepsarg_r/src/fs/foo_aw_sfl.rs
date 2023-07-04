use common::fs_data::{FooAwIn, FooAwOut, FooAwSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn_web, CfgRefCellId, RcPinFnWeb};
use std::time::Duration;
use tokio::time::sleep;

pub type FooAwSflT = RcPinFnWeb<FooAwIn, FooAwOut>;

pub type FooAwSflCfg = CfgRefCellId<FooAwSflCfgInfo>;

#[derive(Clone)]
pub struct FooAwSflDeps {
    pub bar_aw_bf: RcPinFnWeb<u64, String>,
}

pub fn foo_aw_sfl_c(cfg: FooAwSflCfg, deps: FooAwSflDeps) -> FooAwSflT {
    let f = move |input: FooAwIn| {
        let c = cfg.get_cfg();
        let d = deps.clone();
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
