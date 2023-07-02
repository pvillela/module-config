use common::fs_data::BarAiBfCfgInfo;
use common::fs_util::bar_core;
use std::time::Duration;
use tokio::time::sleep;

pub struct BarAiBfS {
    pub cfg: BarAiBfCfgInfo,
}

impl BarAiBfS {
    pub async fn run(&self, sleep_millis: u64) -> String {
        sleep(Duration::from_millis(sleep_millis)).await;

        let cfg = &self.cfg;
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    }
}
