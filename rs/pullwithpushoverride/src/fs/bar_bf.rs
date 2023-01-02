use crate::config::cfg_src::{makeCfgSrc, CfgSrc};
use crate::fs::bar_bf_cfg_adapter::barBfCfgAdapter;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static barBfCfgSrc: Lazy<Mutex<CfgSrc<'static, BarBfCfgInfo>>> =
    Lazy::new(|| Mutex::new(makeCfgSrc(Some(barBfCfgAdapter))));

// lazy_static! {
//     pub static ref barBfCfgSrc: Mutex<CfgSrc<'static, BarBfCfgInfo>> =
//         Mutex::new(makeCfgSrc(Some(barBfCfgAdapter)));
// }

pub fn barBf() {
    println!("{:?}", barBfCfgSrc.lock().unwrap().get());
}
