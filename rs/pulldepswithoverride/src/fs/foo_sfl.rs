use super::bar_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{static_ref_with_override, CfgOvd, CfgRefCellRc, RefreshMode};
use once_cell::sync::{Lazy, OnceCell};

type FooSflCfg = CfgRefCellRc<FooSflCfgInfo>;

pub type FooSflCfgOvd = CfgOvd<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let cfg = FOO_SFL_CFG.with(|c| c.get_cfg());
    let FooSflDeps { bar_bf } = &FOO_SFL_DEPS as &FooSflDeps;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_ret = bar_bf();
    foo_core(a, b, bar_ret)
}

pub static FOO_SFL_DEPS: Lazy<&FooSflDeps> =
    Lazy::new(|| static_ref_with_override(FOO_SFL_DEPS_OVERRIDE.get(), FooSflDeps { bar_bf }));

thread_local! {
pub static FOO_SFL_CFG: FooSflCfg =
    FooSflCfg::new_boxed_with_cfg_adapter_and_override(
        FOO_SFL_CFG_OVERRIDE.get(),
        get_app_configuration,
        foo_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
    )
}

pub static FOO_SFL_CFG_OVERRIDE: OnceCell<FooSflCfgOvd> = OnceCell::new();
pub static FOO_SFL_DEPS_OVERRIDE: OnceCell<FooSflDeps> = OnceCell::new();

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
