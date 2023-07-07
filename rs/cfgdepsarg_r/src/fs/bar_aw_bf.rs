use common::fs_data::BarAwBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDeps, CfgRefCellId, PinFnWeb};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAwBfT = PinFnWeb<u64, String>;

pub type BarAwBfCfg = CfgRefCellId<BarAwBfCfgInfo>;

pub type BarAwBfS = CfgDeps<BarAwBfCfg, ()>;

pub async fn bar_aw_bf_c(s: impl Deref<Target = BarAwBfS>, sleep_millis: u64) -> String {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
