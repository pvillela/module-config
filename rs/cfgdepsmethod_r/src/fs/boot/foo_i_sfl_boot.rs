use crate::fs::boot::get_bar_i_bf_s;
use crate::fs::{FooISflDeps, FooISflS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooISflCfgInfo;
use std::sync::{Arc, OnceLock};

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn get_foo_i_sfl_s_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    deps: FooISflDeps,
) -> FooISflS {
    FooISflS {
        cfg: foo_i_sfl_cfg_adapter(&app_cfg_src()),
        deps,
    }
}

pub fn get_foo_i_sfl_s() -> &'static FooISflS {
    static FOO_A_SFL_S: OnceLock<FooISflS> = OnceLock::new();
    FOO_A_SFL_S.get_or_init(|| {
        get_foo_i_sfl_s_with_app_cfg(
            get_app_configuration,
            FooISflDeps {
                bar_i_bf_s: get_bar_i_bf_s(),
            },
        )
    })
}
