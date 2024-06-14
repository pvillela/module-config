use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct BarAcBfCfgInfo {
    pub u: i32,
    pub v: &'static str,
}

pub const BAR_AC_BF_CFG_INFO: BarAcBfCfgInfo = BarAcBfCfgInfo {
    u: 84,
    v: "constant_config_info",
};

pub async fn bar_ac_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = &BAR_AC_BF_CFG_INFO;
    let u = cfg.u + 1;
    let v = cfg.v.to_string() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
