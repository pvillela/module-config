use super::{bar_a_bf_boot_lr, BarABfT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    box_pin_async_fn, cfg_deps_a_boot, cfg_deps_a_boot_lr, ref_pin_async_fn, AppCfg, CfgArcSwapArc,
    CfgDeps, PinFn,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type FooASflT = PinFn<FooAIn, FooAOut>;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf: Box<BarABfT>,
}

pub type FooASflS = CfgDeps<FooASflCfg, FooASflDeps>;

#[instrument(level = "trace", skip(s))]
pub async fn foo_a_sfl_c(s: impl Deref<Target = FooASflS>, input: FooAIn) -> FooAOut {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_a_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot_a].
/// Returns a boxed foo_a_sfl closure.
pub fn foo_a_sfl_boot_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> Box<FooASflT> {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_a_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );
    let deps = FooASflDeps {
        bar_a_bf: fs::bar_a_bf_boot(app_cfg),
    };
    let foo_a_sfl_s = Arc::new(FooASflS { cfg, deps });
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a boxed foo_a_sfl closure.
pub fn foo_a_sfl_boot(app_cfg: AppCfg<AppCfgInfo>) -> Box<FooASflT> {
    // Using variable definition below in cfg_deps_a_boot call causes rust-analyzer VSCode plugin "inlayHint failed".
    // let cfg_factory = FooASflCfg::new_boxed_with_cfg_adapter;
    let deps = FooASflDeps {
        bar_a_bf: Box::new(fs::bar_a_bf_boot(app_cfg.clone())),
    };
    cfg_deps_a_boot(
        foo_a_sfl_c,
        FooASflCfg::new_boxed_with_cfg_adapter,
        foo_a_sfl_cfg_adapter,
        app_cfg,
        deps,
    )
}

/// Coded without use of [cfg_deps_boot_a_lr].
/// Returns a leaked static reference to a foo_a_sfl closure.
/// The benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_a_sfl_boot_lr_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> &'static FooASflT {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_a_sfl_cfg_adapter,
        app_cfg1.refresh_mode.clone(),
    );
    let deps = FooASflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg)),
    };
    let foo_a_sfl_s: &FooASflS = Box::leak(Box::new(FooASflS { cfg, deps }));
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s, input);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a foo_a_sfl closure.
/// The benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_a_sfl_boot_lr(app_cfg: AppCfg<AppCfgInfo>) -> &'static FooASflT {
    let deps = FooASflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg.clone())),
    };
    cfg_deps_a_boot_lr(
        foo_a_sfl_c,
        FooASflCfg::new_boxed_with_cfg_adapter,
        foo_a_sfl_cfg_adapter,
        app_cfg,
        deps,
    )
}
