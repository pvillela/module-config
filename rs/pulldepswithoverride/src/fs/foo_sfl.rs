use super::bar_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{CfgDepsDefault, RefreshMode};

type FooSflCfgInfo = common::fs_data::FooSflCfgInfo;

#[derive(Clone, Debug)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let (cfg, FooSflDeps { bar_bf }) = FOO_SFL_CFG_DEPS.with(|c| c.get_cfg_deps());
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_ret = bar_bf();
    foo_core(a, b, bar_ret)
}

thread_local! {
pub static FOO_SFL_CFG_DEPS: CfgDepsDefault<FooSflCfgInfo, FooSflDeps> =
    CfgDepsDefault::new_with_cfg_adapter(
        get_app_configuration,
        foo_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
        // RefreshMode::Refreshable(Duration::from_millis(999)),
        FooSflDeps { bar_bf },
    );
}

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
