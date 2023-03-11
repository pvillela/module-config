use common::{fs_util::bar_core, fwk::CfgDepsArc};
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_BF_CFG_DEPS: OnceCell<CfgDepsArc<BarBfCfgInfo, ()>> = OnceCell::new();

pub fn bar_bf() -> String {
    let (cfg, _) = CfgDepsArc::get_from_once_cell(&BAR_BF_CFG_DEPS);
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
