use crate::fs::foo_sfl::{FooSflCfgInfo, FOO_SFL_CFG_SRC};
use crate::fwk::{adapt_by_ref, RefreshMode};
use common::config::AppCfgInfo;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_adapt_cfg_src(
    origin: impl Fn() -> AppCfgInfo + 'static + Send + Sync,
    refresh_mode: RefreshMode,
) {
    adapt_by_ref(origin, foo_sfl_cfg_adapter, refresh_mode, &FOO_SFL_CFG_SRC);
}
