use common::{
    config::{get_app_configuration, AppCfgInfo},
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDeps, CfgRefCellRc, RefreshMode},
};

use super::bar_bf;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflT = fn() -> String;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = FOO_SFL_CFG_DEPS.get_cfg().get_cfg();

    let cfg = FOO_SFL_CFG_TL.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = FOO_SFL_CFG_DEPS.get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}

pub static FOO_SFL_CFG_DEPS: CfgDeps<FooSflCfg, FooSflDeps> = CfgDeps::init(
    || {
        FooSflCfg::new_boxed_with_cfg_adapter(
            get_app_configuration, // use `|| todo!()` before get_app_configuration exists
            foo_sfl_cfg_adapter,   // use `|_| todo!()` before foo_sfl_cfg_adapter exists
            RefreshMode::NoRefresh,
        )
    },
    || {
        FooSflDeps {
            // bar_bf: || todo!(), // do this before bar_bf exists
            bar_bf, // replace above with this after bar_bf has been created
        }
    },
);

thread_local! {
    pub static FOO_SFL_CFG_TL: CfgRefCellRc<FooSflCfgInfo> = cfg_to_thread_local(FOO_SFL_CFG_DEPS.get_cfg());
}

pub fn get_foo_sfl_raw(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let _ = FOO_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_SFL_CFG_DEPS.set_deps_lenient(deps);
    foo_sfl
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
