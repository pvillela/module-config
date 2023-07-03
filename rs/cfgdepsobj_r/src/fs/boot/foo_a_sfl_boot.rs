use crate::fs::boot::{get_bar_a_bf_d_cached, get_bar_a_bf_d_no_refresh};
use crate::fs::{FooASflCfg, FooASflDeps, FooASflS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooASflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn get_foo_a_sfl_s_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
    deps: FooASflDeps,
) -> FooASflS {
    FooASflS {
        cfg: FooASflCfg::new_boxed_with_cfg_adapter(
            app_cfg_src,
            foo_a_sfl_cfg_adapter,
            refresh_mode,
        ),
        deps,
    }
}

pub fn get_foo_a_sfl_s_no_refresh() -> &'static FooASflS {
    static FOO_A_SFL_S: OnceLock<FooASflS> = OnceLock::new();
    FOO_A_SFL_S.get_or_init(|| {
        get_foo_a_sfl_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::NoRefresh,
            FooASflDeps {
                bar_a_bf_d: get_bar_a_bf_d_no_refresh(),
            },
        )
    })
}

pub fn get_foo_a_sfl_s_cached() -> &'static FooASflS {
    static FOO_A_SFL_S_CACHED: OnceLock<FooASflS> = OnceLock::new();
    FOO_A_SFL_S_CACHED.get_or_init(|| {
        get_foo_a_sfl_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(150)),
            FooASflDeps {
                bar_a_bf_d: get_bar_a_bf_d_cached(),
            },
        )
    })
}
