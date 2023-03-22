use super::bar_a_bf_boot;
use crate::fs::{foo_a_sfl_c, FooASflCfgDeps, FooASflDeps, FooASflT};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::RefreshMode;
use once_cell::sync::OnceCell;
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub static FOO_A_SFL_CFG_INFO_OVERRIDE: OnceCell<FooASflCfgInfo> = OnceCell::new();

pub fn foo_a_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooASflT {
    let foo_a_sfl_cfg_deps = FooASflCfgDeps::new_boxed_with_const_or_cfg_adapter(
        FOO_A_SFL_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        foo_a_sfl_cfg_adapter,
        refresh_mode.clone(),
        FooASflDeps {
            bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode),
        },
    );
    foo_a_sfl_c(foo_a_sfl_cfg_deps)
}
