use super::bar_ai_bf_boot;
use crate::fs::{foo_ai_sfl_c, FooAiSflDeps, FooAiSflS, FooAiSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooAiSflCfgInfo;
use common::fwk::arc_pin_async_fn;
use std::sync::Arc;

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_ai_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> FooAiSflT {
    let cfg = foo_ai_sfl_cfg_adapter(&app_cfg());
    let deps = FooAiSflDeps {
        bar_ai_bf: bar_ai_bf_boot(app_cfg),
    };
    let foo_ai_sfl_s = Arc::new(FooAiSflS { cfg, deps });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    arc_pin_async_fn(f)
}
