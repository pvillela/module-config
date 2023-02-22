use crate::config::AppCfgInfo;
use crate::fs::foo_sfl::{FooSflCfgInfo, FOO_SFL_CFG_DEPS};
use crate::fs::FooSflDeps;
use crate::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: FooSflDeps,
) {
    CfgDeps::set_with_cfg_adapter(
        &FOO_SFL_CFG_DEPS,
        origin,
        foo_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}
