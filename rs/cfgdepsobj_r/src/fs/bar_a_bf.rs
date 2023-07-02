use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgArcSwapArc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub struct BarABfS {
    pub cfg: BarABfCfg,
}

impl BarABfS {
    pub async fn run(&self, sleep_millis: u64) -> String {
        sleep(Duration::from_millis(sleep_millis)).await;

        let cfg = self.cfg.get_cfg();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    }
}
