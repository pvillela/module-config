use crate::fs::bar_bf::bar_bf;
use crate::fwk::cfg_src::CfgSrc;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct FooSflCfgInfo {
    pub x: String,
}

pub static FOO_SFL_CFG_SRC: Lazy<ArcSwap<CfgSrc<FooSflCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::nil()));

pub fn foo_sfl() {
    println!("fooSflCfgSrc().x: {}", FOO_SFL_CFG_SRC.load().get().x);
    bar_bf();
}
