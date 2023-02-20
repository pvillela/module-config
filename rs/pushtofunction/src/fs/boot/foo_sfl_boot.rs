use crate::config::AppCfgInfo;
use crate::fs::{foo_sfl_c, FooSflCfgInfo, FooSflCfgSrc, FooSflT};
use crate::fwk::const_or_adapt_by_ref;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use super::bar_bf_boot;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        x: app_cfg.x.clone(),
    }
}

pub static FOO_SFL_CFG_INFO_OVERRIDE: OnceCell<FooSflCfgInfo> = OnceCell::new();

pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> FooSflT {
    let get = const_or_adapt_by_ref(
        FOO_SFL_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        foo_sfl_cfg_adapter,
    );
    let foo_sfl_cfg_src = FooSflCfgSrc {
        get,
        bar: bar_bf_boot(app_cfg),
    };
    foo_sfl_c(foo_sfl_cfg_src)
}
