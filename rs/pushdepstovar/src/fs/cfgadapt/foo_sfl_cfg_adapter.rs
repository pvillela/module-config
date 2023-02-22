use crate::config::AppCfgInfo;
use crate::fs::foo_sfl::{FooSflCfgInfo, FOO_SFL_CFG_SRC};
use crate::fs::FooSflDeps;
use crate::fwk::{adapt_by_ref, RefreshMode};
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
    adapt_by_ref(
        origin,
        foo_sfl_cfg_adapter,
        refresh_mode,
        deps,
        &FOO_SFL_CFG_SRC,
    );
}
