use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{get_from_once_cell, CfgDef, CfgRefCellRc},
};
use once_cell::sync::OnceCell;

pub type FooSflCfg = CfgRefCellRc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = get_from_once_cell(&FOO_SFL_DEPS);
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

thread_local! {
static FOO_SFL_CFG_TL: FooSflCfg =
    FooSflCfg::new_from_once_cell_def(
        &FOO_SFL_CFG_DEF,
    )
}

pub static FOO_SFL_CFG_DEF: OnceCell<CfgDef<FooSflCfgInfo>> = OnceCell::new();
pub static FOO_SFL_DEPS: OnceCell<FooSflDeps> = OnceCell::new();
