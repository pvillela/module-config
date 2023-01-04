use crate::config::cfg_src::{makeCfgSrc, CfgSrc};
use crate::fs::bar_bf_cfg_adapter::{barBfCfgAdapter, barBfCfgAdapter_arc};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static barBfCfgSrc_arc: Lazy<ArcSwap<CfgSrc<Arc<BarBfCfgInfo>>>> =
    Lazy::new(|| ArcSwap::from_pointee(makeCfgSrc(Some(barBfCfgAdapter_arc))));

pub fn barBf_arc() {
    println!("barBf_arc(): {:?}", barBfCfgSrc_arc.load().get());
}

pub static barBfCfgSrc: Lazy<ArcSwap<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(makeCfgSrc(Some(barBfCfgAdapter))));

pub fn barBf() {
    println!("barBf(): {:?}", barBfCfgSrc.load().get());
}
