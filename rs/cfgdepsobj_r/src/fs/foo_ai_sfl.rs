use super::BarAiBfS;
use common::{
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
};
use std::time::Duration;
use tokio::time::sleep;

pub struct FooAiSflDeps {
    pub bar_ai_bf_s: &'static BarAiBfS,
}

pub struct FooAiSflS {
    pub cfg: FooAiSflCfgInfo,
    pub deps: FooAiSflDeps,
}

impl FooAiSflS {
    pub async fn run(&self, input: FooAiIn) -> FooAiOut {
        let FooAiIn { sleep_millis } = input;
        let FooAiSflDeps { bar_ai_bf_s } = self.deps;
        let cfg = &self.cfg;
        let a = cfg.a.clone();
        let b = cfg.b;

        sleep(Duration::from_millis(sleep_millis)).await;
        let bar_res = bar_ai_bf_s.run(0).await;
        let res = foo_core(a, b, bar_res);
        FooAiOut { res }
    }
}
