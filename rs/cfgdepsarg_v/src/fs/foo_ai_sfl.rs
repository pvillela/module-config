use common::fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::ArcPinFn;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAiSflT = ArcPinFn<FooAiIn, FooAiOut>;

// #[derive(Clone)]
pub struct FooAiSflDeps {
    pub bar_ai_bf: ArcPinFn<u64, String>,
}

pub struct FooAiSflS {
    pub cfg: FooAiSflCfgInfo,
    pub deps: FooAiSflDeps,
}

pub async fn foo_ai_sfl_c(s: FooAiSflS, input: FooAiIn) -> FooAiOut {
    let c = &s.cfg;
    let d = &s.deps;
    let FooAiIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_ai_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAiOut { res }
}
