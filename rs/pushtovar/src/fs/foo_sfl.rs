use crate::fs::bar_bf::bar_bf;
use crate::fwk::CfgSrc;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub static FOO_SFL_CFG_SRC: OnceCell<CfgSrc<FooSflCfgInfo>> = OnceCell::new();

pub fn foo_sfl() -> String {
    let cfg = CfgSrc::get_from_static(&FOO_SFL_CFG_SRC);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
}
