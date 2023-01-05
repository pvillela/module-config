use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::BarBfCfgInfo;

use super::foo_sfl::FooSflCfgInfo;

pub fn fooSflCfgAdapter(appCfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: appCfg.x.clone(),
    }
}
