use crate::fwk::CfgSrc;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_SRC: OnceLock<CfgSrc<BarBfCfgInfo>> = OnceLock::new();

pub fn bar_bf() -> String {
    let cfg = CfgSrc::get_from_static(&BAR_BF_CFG_SRC);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
