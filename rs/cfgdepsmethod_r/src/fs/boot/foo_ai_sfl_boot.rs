use super::get_bar_ai_bf_with_app_cfg;
use crate::fs::{get_foo_ai_sfl_raw, FooAiSflDeps, FooAiSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooAiSflCfgInfo;
use std::sync::Arc;

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_ai_sfl_wtih_app_cfg(app_cfg_src: fn() -> Arc<AppCfgInfo>) -> FooAiSflT {
    // A stereotype should initialize its dependencies.
    let bar_ai_bf = get_bar_ai_bf_with_app_cfg(app_cfg_src);
    let deps = FooAiSflDeps { bar_ai_bf };
    get_foo_ai_sfl_raw(foo_ai_sfl_cfg_adapter(&app_cfg_src()), deps)
}
