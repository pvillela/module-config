use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgArcSwapArc, CfgDeps, PinFn};
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfT = PinFn<u64, String>;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfS = CfgDeps<BarABfCfg, ()>;

pub async fn bar_a_bf_c(s: impl AsRef<BarABfS>, sleep_millis: u64) -> String {
    let cfg = s.as_ref().cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
