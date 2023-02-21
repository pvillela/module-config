use crate::fwk::CfgSrc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_SRC: OnceCell<CfgSrc<BarBfCfgInfo>> = OnceCell::new();

pub fn bar_bf() -> String {
    let cfg = CfgSrc::get_from_static(&BAR_BF_CFG_SRC);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
