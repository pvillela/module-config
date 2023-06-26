use crate::fwk::CfgDeps;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_DEPS: OnceLock<CfgDeps<BarBfCfgInfo, ()>> = OnceLock::new();

pub fn bar_bf() -> String {
    let (cfg, _) = CfgDeps::get(&BAR_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
