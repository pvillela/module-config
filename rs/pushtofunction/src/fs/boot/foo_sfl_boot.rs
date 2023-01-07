use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::foo_sfl::{foo_sfl_c, FooSflCfgInfo, FooSflCfgSrc, FooSflT};
use crate::fwk::lift_to_nullary::lift_to_nullary;
use crate::fwk::lift_to_nullary::DressedCfgAdapter;
use once_cell::sync::Lazy;
use std::sync::Arc;

use super::bar_bf_boot::bar_bf_boot;

fn foo_sfl_cfg_adapter0(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: app_cfg.x.clone(),
    }
}

pub static FOO_SFL_CFG_ADAPTER: Lazy<DressedCfgAdapter<AppCfgInfo, FooSflCfgInfo>> =
    Lazy::new(|| lift_to_nullary(foo_sfl_cfg_adapter0));

pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> FooSflT {
    let foo_sfl_cfg_src = FooSflCfgSrc {
        get: (FOO_SFL_CFG_ADAPTER.load())(app_cfg),
        bar: bar_bf_boot(app_cfg),
    };
    return foo_sfl_c(foo_sfl_cfg_src);
}
