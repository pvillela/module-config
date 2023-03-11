use common::{fs_util::bar_core, fwk::CfgDepsArc};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BarABfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_A_BF_CFG_DEPS: OnceCell<CfgDepsArc<BarABfCfgInfo, ()>> = OnceCell::new();

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, _) = CfgDepsArc::get_from_once_cell(&BAR_A_BF_CFG_DEPS);
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
