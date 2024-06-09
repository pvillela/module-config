use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    box_pin_async_fn, cfg_deps_ai_boot, cfg_deps_ai_boot_lr, ref_pin_async_fn, CfgDeps, PinFn,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use super::{bar_ai_bf_boot_lr, BarAiBfT};

pub type FooAiSflT = PinFn<FooAiIn, FooAiOut>;

// #[derive(Clone)]
pub struct FooAiSflDeps {
    pub bar_ai_bf: Box<BarAiBfT>,
}

pub type FooAiSflS = CfgDeps<FooAiSflCfgInfo, FooAiSflDeps>;

pub async fn foo_ai_sfl_c(s: impl Deref<Target = FooAiSflS>, input: FooAiIn) -> FooAiOut {
    let c = &s.cfg;
    let d = &s.deps;
    let FooAiIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_ai_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAiOut { res }
}

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
        bar_ai_bf: fs::bar_ai_bf_boot(app_cfg),
    };
    let foo_ai_sfl_s = Arc::new(FooAiSflS { cfg, deps });
    let f = move |input| foo_ai_sfl_c(foo_ai_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a boxed foo_ai_sfl_closure.
pub fn foo_ai_sfl_boot(app_cfg: &AppCfgInfo) -> Box<FooAiSflT> {
    let deps = FooAiSflDeps {
        bar_ai_bf: fs::bar_ai_bf_boot(app_cfg),
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
