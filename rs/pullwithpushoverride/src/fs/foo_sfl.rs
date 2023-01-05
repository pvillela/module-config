use crate::config::cfg_src::CfgSrc;
use crate::fs::{bar_bf::bar_bf, foo_sfl_cfg_adapter::foo_sfl_cfg_adapter};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct FooSflCfgInfo {
    pub x: String,
}

pub static FOO_SFL_CFG_SRC: Lazy<ArcSwap<CfgSrc<FooSflCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::from_adapter(foo_sfl_cfg_adapter)));

pub fn foo_sfl() {
    println!("fooSflCfgSrc().x: {}", FOO_SFL_CFG_SRC.load().get().x);
    bar_bf();
}
