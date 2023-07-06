use super::bar_a_bf_boot;
use crate::fs::{foo_a_sfl_c, FooASflCfg, FooASflDeps, FooASflS, FooASflT};
use bar_a_bf_boot::bar_a_bf_boot_xr;
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{box_pin_async_fn, RefreshMode};
use std::sync::{Arc, OnceLock};

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

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn foo_a_sfl_boot_r(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooASflT> {
    static FOO_A_SFL_S: OnceLock<FooASflS> = OnceLock::new();
    let foo_a_sfl_s = FOO_A_SFL_S.get_or_init(|| {
        let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
            app_cfg,
            foo_a_sfl_cfg_adapter,
            refresh_mode.clone(),
        );
        let deps = FooASflDeps {
            bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode.clone()),
        };
        FooASflS { cfg, deps }
    });
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for this and its dependencies
// for each call to the returned function.
pub fn foo_a_sfl_boot_xr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooASflT> {
    static FOO_A_SFL_S_X: OnceLock<FooASflS> = OnceLock::new();
    let foo_a_sfl_s = FOO_A_SFL_S_X.get_or_init(|| {
        let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
            app_cfg,
            foo_a_sfl_cfg_adapter,
            refresh_mode.clone(),
        );
        let deps = FooASflDeps {
            bar_a_bf: bar_a_bf_boot_xr(app_cfg, refresh_mode.clone()),
        };
        FooASflS { cfg, deps }
    });
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s.clone(), input);
    box_pin_async_fn(f)
}
