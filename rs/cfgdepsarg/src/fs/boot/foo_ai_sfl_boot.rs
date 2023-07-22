use super::bar_ai_bf_boot;
use crate::fs::{foo_ai_sfl_c, FooAiSflDeps, FooAiSflS, FooAiSflT};
use bar_ai_bf_boot::bar_ai_bf_boot_lr;
use common::fs_data::FooAiSflCfgInfo;
use common::fwk::{box_pin_async_fn, cfg_deps_ai_boot, cfg_deps_ai_boot_lr};
use common::{config::AppCfgInfo, fwk::ref_pin_async_fn};
use std::sync::Arc;

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot_ai].
/// Returns a boxed foo_ai_sfl_closure.
pub fn foo_ai_sfl_boot_by_hand(app_cfg: &AppCfgInfo) -> Box<FooAiSflT> {
    let cfg = foo_ai_sfl_cfg_adapter(&app_cfg);
    let deps = FooAiSflDeps {
        bar_ai_bf: bar_ai_bf_boot(app_cfg),
    };
    let foo_ai_sfl_s = Arc::new(FooAiSflS { cfg, deps });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a boxed foo_ai_sfl_closure.
pub fn foo_ai_sfl_boot(app_cfg: &AppCfgInfo) -> Box<FooAiSflT> {
    let deps = FooAiSflDeps {
        bar_ai_bf: bar_ai_bf_boot(app_cfg),
    };
    cfg_deps_ai_boot(foo_ai_sfl_c, foo_ai_sfl_cfg_adapter, app_cfg, deps)
}

/// Coded without use of [cfg_deps_boot_ai].
/// Returns a leaked static reference to a foo_ai_sfl closure.
/// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_ai_sfl_boot_lr_by_hand(app_cfg: &AppCfgInfo) -> &'static FooAiSflT {
    let cfg = foo_ai_sfl_cfg_adapter(&app_cfg);
    let deps = FooAiSflDeps {
        bar_ai_bf: Box::new(bar_ai_bf_boot_lr(app_cfg)),
    };
    let foo_ai_sfl_s: &FooAiSflS = Box::leak(Box::new(FooAiSflS { cfg, deps }));
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s, input);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a foo_ai_sfl closure.
/// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_ai_sfl_boot_lr(app_cfg: &AppCfgInfo) -> &'static FooAiSflT {
    let deps = FooAiSflDeps {
        bar_ai_bf: Box::new(bar_ai_bf_boot_lr(app_cfg)),
    };
    cfg_deps_ai_boot_lr(foo_ai_sfl_c, foo_ai_sfl_cfg_adapter, app_cfg, deps)
}
