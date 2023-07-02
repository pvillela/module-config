use super::get_bar_i_bf_with_app_cfg;
use crate::fs::{get_foo_i_sfl_raw, FooISflDeps, FooISflT};
use common::config::AppCfgInfo;
use common::fs_data::FooISflCfgInfo;

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_i_sfl_with_app_cfg(app_cfg: &AppCfgInfo) -> FooISflT {
    // A stereotype should initialize its dependencies.
    let bar_i_bf = get_bar_i_bf_with_app_cfg(app_cfg);
    let deps = FooISflDeps { bar_i_bf };
    get_foo_i_sfl_raw(foo_i_sfl_cfg_adapter(app_cfg), deps)
}
