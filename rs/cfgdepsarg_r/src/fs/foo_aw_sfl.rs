use super::BarAwBfT;
use common::fs_data::{FooAwIn, FooAwOut, FooAwSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{CfgDeps, CfgRefCellId, PinFnWeb};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAwSflT = PinFnWeb<FooAwIn, FooAwOut>;

pub type FooAwSflCfg = CfgRefCellId<FooAwSflCfgInfo>;

// #[derive(Clone)]
pub struct FooAwSflDeps {
    pub bar_aw_bf: Box<BarAwBfT>,
}

pub type FooAwSflS = CfgDeps<FooAwSflCfg, FooAwSflDeps>;

pub async fn foo_aw_sfl_c(s: impl Deref<Target = FooAwSflS>, input: FooAwIn) -> FooAwOut {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAwIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_aw_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAwOut { res }
}
