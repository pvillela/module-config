use crate::fwk::CfgSrc;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static BAR_BF_CFG_SRC: Lazy<ArcSwap<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::nil()));

pub fn bar_bf() {
    println!("barBf(): {:?}", BAR_BF_CFG_SRC.load().get());
}
