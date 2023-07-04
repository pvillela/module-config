use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{ArcPinFn, CfgArcSwapArc, CfgDeps};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfT = ArcPinFn<u64, String>;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfS = CfgDeps<BarABfCfg, ()>;

pub async fn bar_a_bf_c(s: Arc<BarABfS>, sleep_millis: u64) -> String {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
