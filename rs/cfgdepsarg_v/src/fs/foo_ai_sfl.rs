use common::fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{BoxPinFn, CfgDeps};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAiSflT = BoxPinFn<FooAiIn, FooAiOut>;

// #[derive(Clone)]
pub struct FooAiSflDeps {
    pub bar_ai_bf: BoxPinFn<u64, String>,
}

pub type FooAiSflS = CfgDeps<FooAiSflCfgInfo, FooAiSflDeps>;

pub async fn foo_ai_sfl_c(s: Arc<FooAiSflS>, input: FooAiIn) -> FooAiOut {
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
