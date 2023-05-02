use super::bar_bf_init;
use crate::fs::{bar_bf, FooSflCfg, FooSflDeps, FOO_SFL_CFG, FOO_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::{init_option, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_init(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    // A stereotype should initialize its dependencies.
    bar_bf_init(origin, refresh_mode.clone());
    unsafe {
        init_option(
            &mut FOO_SFL_CFG,
            FooSflCfg::new_boxed_with_cfg_adapter(origin, foo_sfl_cfg_adapter, refresh_mode),
        );
        init_option(&mut FOO_SFL_DEPS, FooSflDeps { bar_bf });
    }
}

pub fn foo_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_millis: u64) {
    foo_sfl_init(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(refresh_millis)),
    );
}

pub fn foo_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    foo_sfl_init(app_cfg_src, RefreshMode::NoRefresh);
}
