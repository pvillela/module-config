use crate::fs::{foo_sfl_c, FooSflCfg, FooSflDeps, FooSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

use super::bar_bf_boot;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooSflT {
    let cfg =
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg, foo_sfl_cfg_adapter, refresh_mode.clone());
    let deps = FooSflDeps {
        bar_bf: bar_bf_boot(app_cfg, refresh_mode),
    };
    foo_sfl_c(cfg, deps)
}
