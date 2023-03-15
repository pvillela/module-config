use crate::fs::{foo_sfl_c, FooSflCfgDeps, FooSflDeps, FooSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::RefreshMode;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use super::bar_bf_boot;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub static FOO_SFL_CFG_INFO_OVERRIDE: OnceCell<FooSflCfgInfo> = OnceCell::new();

pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooSflT {
    let foo_sfl_cfg_deps = FooSflCfgDeps::new_with_const_or_cfg_adapter(
        FOO_SFL_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        foo_sfl_cfg_adapter,
        refresh_mode.clone(),
        FooSflDeps {
            bar_bf: bar_bf_boot(app_cfg, refresh_mode),
        },
    );
    foo_sfl_c(foo_sfl_cfg_deps)
}
