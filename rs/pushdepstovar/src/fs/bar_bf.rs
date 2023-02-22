use crate::fwk::CfgDepsSrc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_SRC: OnceCell<CfgDepsSrc<BarBfCfgInfo, ()>> = OnceCell::new();

pub fn bar_bf() -> String {
    let (cfg, _) = CfgDepsSrc::get_from_static(&BAR_BF_CFG_SRC);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
