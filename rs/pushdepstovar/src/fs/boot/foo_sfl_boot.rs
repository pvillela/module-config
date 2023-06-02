use super::get_bar_bf_with_app_cfg;
use crate::fs::foo_sfl::get_foo_sfl_raw;
use crate::fs::{FooSflCfg, FooSflDeps, FooSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_sfl_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> FooSflT {
    // A stereotype should initialize its dependencies.
    let bar_bf = get_bar_bf_with_app_cfg(app_cfg_src, refresh_mode.clone());
    let deps = FooSflDeps { bar_bf };
    get_foo_sfl_raw(
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
