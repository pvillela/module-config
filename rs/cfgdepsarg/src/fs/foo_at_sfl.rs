use super::BarAtBfT;
use common::fs_data::{FooAtIn, FooAtOut, FooAtSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{AppErr, CfgArcSwapArc, CfgDeps, PinFn2a, PinFn2r, Tx};
use futures::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAtSflT = PinFn2r<FooAtIn, Tx, Result<FooAtOut, AppErr>>;
// impl Fn(S1, &'a S2) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync + 'a
pub type FooAtSflCfg = CfgArcSwapArc<FooAtSflCfgInfo>;

pub struct FooAtSflDeps {
    pub bar_at_bf: Box<
        dyn for<'a> Fn(
            u64,
            &'a Tx,
        )
            -> Pin<Box<dyn Future<Output = Result<String, AppErr>> + Send + Sync>>,
    >,
}

pub type FooAtSflS = CfgDeps<FooAtSflCfg, FooAtSflDeps>;

pub async fn foo_at_sfl_c(
    s: impl Deref<Target = FooAtSflS>,
    input: FooAtIn,
    tx: &Tx,
) -> Result<FooAtOut, AppErr> {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAtIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_at_bf)(0, tx).await.unwrap();
    let res = foo_core(a, b, bar_res) + &tx.dummy("foo_at_sfl_c");
    Ok(FooAtOut { res })
}
