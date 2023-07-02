use super::BarABfS;
use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::CfgArcSwapArc,
};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf_s: &'static BarABfS,
}

pub struct FooASflS {
    pub cfg: FooASflCfg,
    pub deps: FooASflDeps,
}

impl FooASflS {
    pub async fn run(&self, input: FooAIn) -> FooAOut {
        let FooAIn { sleep_millis } = input;
        let FooASflDeps { bar_a_bf_s } = self.deps;
        let cfg = self.cfg.get_cfg();
        let a = cfg.a.clone();
        let b = cfg.b;

        sleep(Duration::from_millis(sleep_millis)).await;
        let bar_res = bar_a_bf_s.run(0).await;
        let res = foo_core(a, b, bar_res);
        FooAOut { res }
    }
}
