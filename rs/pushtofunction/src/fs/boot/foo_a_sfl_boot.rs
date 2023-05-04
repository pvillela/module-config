use super::bar_a_bf_boot;
use crate::fs::{foo_a_sfl_c, FooASflCfgDeps, FooASflDeps, FooASflT};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_a_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooASflT {
    let foo_a_sfl_cfg_deps = FooASflCfgDeps::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_a_sfl_cfg_adapter,
        refresh_mode.clone(),
        FooASflDeps {
            bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode),
        },
    );
    foo_a_sfl_c(foo_a_sfl_cfg_deps)
}
