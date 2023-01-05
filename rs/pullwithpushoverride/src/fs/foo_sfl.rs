use crate::config::cfg_src::CfgSrc;
use crate::fs::{bar_bf::barBf, foo_sfl_cfg_adapter::fooSflCfgAdapter};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct FooSflCfgInfo {
    pub x: String,
}

pub static fooSflCfgSrc: Lazy<ArcSwap<CfgSrc<FooSflCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::from_adapter(fooSflCfgAdapter)));

pub fn fooSfl() {
    println!("fooSflCfgSrc().x: {}", fooSflCfgSrc.load().get().x);
    barBf();
}
