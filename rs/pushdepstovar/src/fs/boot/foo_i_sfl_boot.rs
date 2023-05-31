use super::bar_i_bf_init;
use crate::fs::{bar_i_bf, FooISflDeps, FOO_I_SFL_CFG, FOO_I_SFL_DEPS};
use common::config::AppCfgInfo;
use common::fs_data::FooISflCfgInfo;
use common::fwk::set_once_cell;
use std::sync::Arc;

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_i_sfl_init(origin: fn() -> Arc<AppCfgInfo>) {
    bar_i_bf_init(origin);
    let _ = set_once_cell(&FOO_I_SFL_CFG, foo_i_sfl_cfg_adapter(&origin()));
    let _ = set_once_cell(&FOO_I_SFL_DEPS, FooISflDeps { bar_i_bf });
}
