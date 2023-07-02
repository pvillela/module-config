use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{CfgArcSwapArc, Pinfn},
};
use std::time::Duration;
use tokio::time::sleep;

use super::BarABfS;

pub type FooASflT = Pinfn<FooAIn, FooAOut>;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps<'a> {
    pub bar_a_bf_s: &'a BarABfS,
}

pub struct FooASflS<'a> {
    pub cfg: FooASflCfg,
    pub deps: FooASflDeps<'a>,
}

impl<'a> FooASflS<'a> {
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
