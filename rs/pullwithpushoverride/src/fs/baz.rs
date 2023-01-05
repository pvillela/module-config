use crate::config::cfg_src::CfgSrc;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct BazCfgInfo {
    pub w: String,
}

pub static bazCfgSrc: Lazy<ArcSwap<CfgSrc<BazCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::<BazCfgInfo>::nil()));

pub fn baz() {
    println!("bazCfgSrc().w.length(): {}", bazCfgSrc.load().get().w.len());
}
