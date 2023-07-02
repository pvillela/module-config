use crate::fs::boot::get_bar_ai_bf_s;
use crate::fs::{FooAiSflDeps, FooAiSflS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooAiSflCfgInfo;
use std::sync::{Arc, OnceLock};

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn get_foo_ai_sfl_s_app_cfg(app_cfg_src: fn() -> Arc<AppCfgInfo>, deps: FooAiSflDeps) -> FooAiSflS {
    FooAiSflS {
        cfg: foo_ai_sfl_cfg_adapter(&app_cfg_src()),
        deps,
    }
}

pub fn get_foo_ai_sfl_s() -> &'static FooAiSflS {
    static FOO_A_SFL_S: OnceLock<FooAiSflS> = OnceLock::new();
    FOO_A_SFL_S.get_or_init(|| {
        get_foo_ai_sfl_s_app_cfg(
            get_app_configuration,
            FooAiSflDeps {
                bar_ai_bf_s: get_bar_ai_bf_s(),
            },
        )
    })
}
