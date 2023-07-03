use crate::fs::boot::{get_bar_bf_d_cached, get_bar_bf_d_no_refresh};
use crate::fs::{foo_sfl_c, FooSflCfg, FooSflD, FooSflDeps, FooSflS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fwk::RefreshMode;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_sfl_s_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
    deps: FooSflDeps,
) -> FooSflS {
    FooSflS {
        cfg: FooSflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_sfl_cfg_adapter, refresh_mode),
        deps,
    }
}

pub fn get_foo_sfl_d_no_refresh() -> FooSflD {
    static FOO_SFL_S: OnceLock<FooSflS> = OnceLock::new();
    let foo_sfl_s = FOO_SFL_S.get_or_init(|| {
        get_foo_sfl_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::NoRefresh,
            FooSflDeps {
                bar_bf_d: get_bar_bf_d_no_refresh(),
            },
        )
    });
    FooSflD {
        s: foo_sfl_s,
        f: foo_sfl_c,
    }
}

pub fn get_foo_sfl_d_cached() -> FooSflD {
    static FOO_SFL_S_CACHED: OnceLock<FooSflS> = OnceLock::new();
    let foo_sfl_s = FOO_SFL_S_CACHED.get_or_init(|| {
        get_foo_sfl_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(150)),
            FooSflDeps {
                bar_bf_d: get_bar_bf_d_cached(),
            },
        )
    });
    FooSflD {
        s: foo_sfl_s,
        f: foo_sfl_c,
    }
}
