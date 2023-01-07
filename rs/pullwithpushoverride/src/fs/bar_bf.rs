use crate::config::cfg_src::CfgSrc;
use crate::fs::bar_bf_cfg_adapter::bar_bf_cfg_adapter;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static BAR_BF_CFG_SRC: Lazy<ArcSwap<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::from_adapter(bar_bf_cfg_adapter)));

pub fn bar_bf() {
    println!("barBf(): {:?}", BAR_BF_CFG_SRC.load().get());
}
