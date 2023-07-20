use super::BarAtBfTxT;
use common::fs_data::{FooAtIn, FooAtOut, FooAtSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{AppErr, CfgArcSwapArc, CfgDeps, PinBorrowFn2b2, Tx};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAtSflTxT = PinBorrowFn2b2<FooAtIn, Tx, Result<FooAtOut, AppErr>>;

pub type FooAtSflCfg = CfgArcSwapArc<FooAtSflCfgInfo>;

pub struct FooAtSflDeps {
    pub bar_at_bf: Box<BarAtBfTxT>,
}

pub type FooAtSflS = CfgDeps<FooAtSflCfg, FooAtSflDeps>;

pub async fn foo_at_sfl_c(
    s: impl Deref<Target = FooAtSflS> + Send + Sync,
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
