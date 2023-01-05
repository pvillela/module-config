use crate::config::app_cfg_info::AppCfgInfo;

use super::foo_sfl::FooSflCfgInfo;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: app_cfg.x.clone(),
    }
}
