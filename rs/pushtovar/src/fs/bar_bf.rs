use crate::fwk::CfgSrc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub static BAR_BF_CFG_SRC: OnceCell<CfgSrc<BarBfCfgInfo>> = OnceCell::new();

pub fn bar_bf() {
    println!("barBf(): {:?}", BAR_BF_CFG_SRC.get().unwrap().get());
}
