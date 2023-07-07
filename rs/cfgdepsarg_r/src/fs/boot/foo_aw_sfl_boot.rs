use super::bar_aw_bf_boot;
use crate::fs::{foo_aw_sfl_c, FooAwSflCfg, FooAwSflDeps, FooAwSflS, FooAwSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooAwSflCfgInfo;
use common::fwk::{box_pin_async_fn_web, RefreshMode};
use std::sync::Arc;

fn foo_aw_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAwSflCfgInfo {
    FooAwSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_aw_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooAwSflT> {
    let cfg = FooAwSflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_aw_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooAwSflDeps {
        bar_aw_bf: bar_aw_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_aw_sfl_s = Arc::new(FooAwSflS { cfg, deps });
    let f = move |input| foo_aw_sfl_c(foo_aw_sfl_s.clone(), input);
    box_pin_async_fn_web(f)
}
