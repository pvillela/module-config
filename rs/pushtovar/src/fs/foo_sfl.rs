use crate::fs::bar_bf::bar_bf;
use crate::fwk::CfgSrc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub x: String,
}

pub static FOO_SFL_CFG_SRC: OnceCell<CfgSrc<FooSflCfgInfo>> = OnceCell::new();

pub fn foo_sfl() {
    println!(
        "fooSflCfgSrc().x: {}",
        FOO_SFL_CFG_SRC
            .get()
            .expect("FOO_SFL_CFG_SRC not initialized")
            .get()
            .x
    );
    bar_bf();
}
