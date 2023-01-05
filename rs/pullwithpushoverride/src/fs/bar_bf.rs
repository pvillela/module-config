use crate::config::cfg_src::CfgSrc;
use crate::fs::bar_bf_cfg_adapter::barBfCfgAdapter;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static barBfCfgSrc: Lazy<ArcSwap<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::from_adapter(barBfCfgAdapter)));

pub fn barBf() {
    println!("barBf(): {:?}", barBfCfgSrc.load().get());
}
