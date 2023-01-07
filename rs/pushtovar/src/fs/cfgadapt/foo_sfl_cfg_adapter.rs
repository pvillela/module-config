use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::foo_sfl::{FooSflCfgInfo, FOO_SFL_CFG_SRC};
use crate::fwk::cfg_src::CfgSrcAdaptation;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::ops::Deref;

pub fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: app_cfg.x.clone(),
    }
}

pub static FOO_SFL_CFG_ADAPTATION: Lazy<ArcSwap<CfgSrcAdaptation<AppCfgInfo, FooSflCfgInfo>>> =
    Lazy::new(|| {
        ArcSwap::from_pointee(CfgSrcAdaptation {
            target_src: FOO_SFL_CFG_SRC.deref(),
            adapter: foo_sfl_cfg_adapter,
        })
    });
