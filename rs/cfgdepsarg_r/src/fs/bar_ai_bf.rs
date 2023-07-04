use common::fs_data::BarAiBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDeps, RefPinFn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAiBfT = RefPinFn<u64, String>;

pub type BarAiBfS = CfgDeps<BarAiBfCfgInfo, ()>;

pub async fn bar_ai_bf_c(s: Arc<BarAiBfS>, sleep_millis: u64) -> String {
    let cfg = &s.cfg;
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
