use super::bar_ai_bf_boot;
use crate::fs::{foo_ai_sfl_c, FooAiSflDeps, FooAiSflS, FooAiSflT};
use bar_ai_bf_boot::{bar_ai_bf_boot_lr, bar_ai_bf_boot_xs, get_bar_ai_bf};
use common::config::get_app_configuration;
use common::fs_data::FooAiSflCfgInfo;
use common::fwk::{box_pin_async_fn, cfg_deps_boot_ai_lr};
use common::{config::AppCfgInfo, fwk::ref_pin_async_fn};
use std::sync::{Arc, OnceLock};

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_ai_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooAiSflT> {
    let cfg = foo_ai_sfl_cfg_adapter(&app_cfg());
    let deps = FooAiSflDeps {
        bar_ai_bf: bar_ai_bf_boot(app_cfg),
    };
    let foo_ai_sfl_s = Arc::new(FooAiSflS { cfg, deps });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn foo_ai_sfl_boot_s(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooAiSflT> {
    static FOO_AI_SFL_S: OnceLock<FooAiSflS> = OnceLock::new();
    let foo_ai_sfl_s = FOO_AI_SFL_S.get_or_init(|| {
        let cfg = foo_ai_sfl_cfg_adapter(&app_cfg());
        let deps = FooAiSflDeps {
            bar_ai_bf: bar_ai_bf_boot(app_cfg),
        };
        FooAiSflS { cfg, deps }
    });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for this and its dependencies
// for each call to the returned function.
pub fn foo_ai_sfl_boot_xs(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooAiSflT> {
    static FOO_AI_SFL_S_X: OnceLock<FooAiSflS> = OnceLock::new();
    let foo_ai_sfl_s = FOO_AI_SFL_S_X.get_or_init(|| {
        let cfg = foo_ai_sfl_cfg_adapter(&app_cfg());
        let deps = FooAiSflDeps {
            bar_ai_bf: bar_ai_bf_boot_xs(app_cfg),
        };
        FooAiSflS { cfg, deps }
    });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a leaked static reference to a foo_ai_sfl closure.
/// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_ai_sfl_boot_lr(app_cfg: fn() -> Arc<AppCfgInfo>) -> &'static FooAiSflT {
    let cfg = foo_ai_sfl_cfg_adapter(&app_cfg());
    let deps = FooAiSflDeps {
        bar_ai_bf: Box::new(bar_ai_bf_boot_lr(app_cfg)),
    };
    let foo_ai_sfl_s: &FooAiSflS = Box::leak(Box::new(FooAiSflS { cfg, deps }));
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s, input);
    ref_pin_async_fn(f)
}

pub fn get_foo_ai_sfl() -> &'static FooAiSflT {
    static FOO_AI_SFL: OnceLock<&FooAiSflT> = OnceLock::new();
    FOO_AI_SFL.get_or_init(|| {
        let deps = FooAiSflDeps {
            bar_ai_bf: Box::new(get_bar_ai_bf()),
        };

        cfg_deps_boot_ai_lr(
            foo_ai_sfl_c,
            foo_ai_sfl_cfg_adapter,
            get_app_configuration(),
            deps,
        )
    })
}
