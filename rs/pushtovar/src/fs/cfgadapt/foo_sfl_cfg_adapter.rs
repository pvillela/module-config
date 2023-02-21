use crate::config::AppCfgInfo;
use crate::fs::foo_sfl::{FooSflCfgInfo, FOO_SFL_CFG_SRC};
use crate::fwk::{adapt_by_ref, RefreshMode};
use std::sync::Arc;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: app_cfg.x.clone(),
    }
}

pub fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
) {
    adapt_by_ref(origin, foo_sfl_cfg_adapter, refresh_mode, &FOO_SFL_CFG_SRC);
}
