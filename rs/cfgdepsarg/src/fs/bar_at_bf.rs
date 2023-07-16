use common::fs_data::BarAtBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{AppErr, CfgArcSwapArc, CfgDeps, PinFn2a, PinFn2r, Tx};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;

// TODO: Is Tx Clone? Let's find out about popular Rust persistence packages.

pub type BarAtBfT = PinFn2r<u64, Tx, Result<String, AppErr>>;

pub type BarAtBfCfg = CfgArcSwapArc<BarAtBfCfgInfo>;

pub type BarAtBfS = CfgDeps<BarAtBfCfg, ()>;

pub async fn bar_at_bf_c(
    s: impl Deref<Target = BarAtBfS>,
    sleep_millis: u64,
    tx: &Tx,
) -> Result<String, AppErr> {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    let res = bar_core(u, v) + &tx.dummy("bar_at_bf_c");
    Ok(res)
}
