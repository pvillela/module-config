use super::bar_a_bf_boot;
use crate::fs::{foo_a_sfl_c, FooASflCfg, FooASflDeps, FooASflS, FooASflT};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{box_pin_async_fn, RefreshMode};
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_a_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooASflT> {
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_a_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooASflDeps {
        bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_a_sfl_s = Arc::new(FooASflS { cfg, deps });
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s.clone(), input);
    box_pin_async_fn(f)
}
