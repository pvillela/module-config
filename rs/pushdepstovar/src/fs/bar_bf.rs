use common::fwk::CfgDepsArc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_DEPS: OnceCell<CfgDepsArc<BarBfCfgInfo, ()>> = OnceCell::new();

pub fn bar_bf() -> String {
    let (cfg, _) = CfgDepsArc::get_from_once_cell(&BAR_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}
