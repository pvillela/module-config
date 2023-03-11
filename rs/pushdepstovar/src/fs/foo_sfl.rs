use common::{fs_util::foo_core, fwk::CfgDepsArc};
use once_cell::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Debug, Clone)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub static FOO_SFL_CFG_DEPS: OnceCell<CfgDepsArc<FooSflCfgInfo, FooSflDeps>> = OnceCell::new();

pub fn foo_sfl() -> String {
    let (cfg, FooSflDeps { bar_bf }) = CfgDepsArc::get_from_once_cell(&FOO_SFL_CFG_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}
