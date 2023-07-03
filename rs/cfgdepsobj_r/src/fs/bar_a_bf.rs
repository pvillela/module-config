use std::time::Duration;

use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgArcSwapArc, Dep1a};
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub struct BarABfS {
    pub cfg: BarABfCfg,
}

pub type BarABfD = Dep1a<BarABfS, u64, String>;

pub async fn bar_a_bf_c(s: &BarABfS, sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    let cfg = s.cfg.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
