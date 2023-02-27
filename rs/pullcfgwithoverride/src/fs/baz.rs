use crate::config::CfgSrc;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BazCfgInfo {
    pub w: String,
}

pub static BAZ_CFG_SRC: Lazy<ArcSwap<CfgSrc<BazCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::<BazCfgInfo>::nil()));

pub fn baz() {
    println!(
        "bazCfgSrc().w.length(): {}",
        BAZ_CFG_SRC.load().get().w.len()
    );
}
