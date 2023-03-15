use super::bar_aw_bf_boot;
use crate::fs::{foo_aw_sfl_c, FooAwSflCfgDeps, FooAwSflDeps, FooAwSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooAwSflCfgInfo;
use common::fwk::RefreshMode;
use once_cell::sync::OnceCell;
use std::sync::Arc;

fn foo_aw_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAwSflCfgInfo {
    FooAwSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub static FOO_AW_SFL_CFG_INFO_OVERRIDE: OnceCell<FooAwSflCfgInfo> = OnceCell::new();

pub fn foo_aw_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooAwSflT {
    let foo_aw_sfl_cfg_deps = FooAwSflCfgDeps::new_with_const_or_cfg_adapter(
        FOO_AW_SFL_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        foo_aw_sfl_cfg_adapter,
        refresh_mode.clone(),
        FooAwSflDeps {
            bar_aw_bf: bar_aw_bf_boot(app_cfg, refresh_mode),
        },
    );
    foo_aw_sfl_c(foo_aw_sfl_cfg_deps)
}
