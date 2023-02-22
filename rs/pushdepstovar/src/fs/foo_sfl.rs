use crate::fwk::CfgDeps;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Debug)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub static FOO_SFL_CFG_DEPS: OnceCell<CfgDeps<FooSflCfgInfo, FooSflDeps>> = OnceCell::new();

pub fn foo_sfl() -> String {
    let (cfg, FooSflDeps { bar_bf }) = CfgDeps::get(&FOO_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
}
