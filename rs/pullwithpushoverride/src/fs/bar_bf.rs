use crate::config::cfg_src::{makeCfgSrc, CfgSrc};
use crate::fs::bar_bf_cfg_adapter::barBfCfgAdapter;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static barBfCfgSrc: Lazy<Mutex<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| Mutex::new(makeCfgSrc(Some(barBfCfgAdapter))));

// lazy_static! {
//     pub static ref barBfCfgSrc: Mutex<CfgSrc<'static, BarBfCfgInfo>> =
//         Mutex::new(makeCfgSrc(Some(barBfCfgAdapter)));
// }

pub static barBfCfgSrc1: Lazy<ArcSwap<CfgSrc<BarBfCfgInfo>>> =
    Lazy::new(|| ArcSwap::from_pointee(makeCfgSrc(Some(barBfCfgAdapter))));

pub fn barBf() {
    println!("{:?}", barBfCfgSrc.lock().unwrap().get());
}

pub fn barBf1() {
    println!("{:?}", barBfCfgSrc1.load().get());
}
