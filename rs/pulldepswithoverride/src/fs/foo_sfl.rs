use super::bar_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgRefCellRc, RefreshMode};
use std::sync::OnceLock;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg().get_cfg();

    let cfg = &FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_ret = bar_bf();
    foo_core(a, b, bar_ret)
}

pub static FOO_SFL_CFG: OnceLock<FooSflCfg> = OnceLock::new();

fn get_cfg() -> &'static FooSflCfg {
    FOO_SFL_CFG.get_or_init(|| {
        FooSflCfg::new_boxed_with_cfg_adapter(
            get_app_configuration, // use `|| todo!()` before get_app_configuration exists
            foo_sfl_cfg_adapter,   // use `|_| todo!()` before foo_sfl_cfg_adapter exists
            RefreshMode::NoRefresh,
        )
    })
}

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_to_thread_local(get_cfg());
}

pub static FOO_SFL_DEPS: OnceLock<FooSflDeps> = OnceLock::new();

fn get_deps() -> &'static FooSflDeps {
    FOO_SFL_DEPS.get_or_init(|| {
        FooSflDeps {
            // bar_bf: || todo!(), // do this before bar_bf exists
            bar_bf, // replace above with this after bar_bf has been created
        }
    })
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
