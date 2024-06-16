use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{box_pin_async_fn, CfgDeps, PinFn};
use futures::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarArBfT = PinFn<u64, String>;

pub struct BarArBfCfgInfo<'a> {
    pub u: i32,
    pub v: &'a str,
}

pub type BarArBfS = CfgDeps<fn() -> AppCfgInfo, ()>;

pub async fn bar_ar_bf_c(s: impl Deref<Target = BarArBfS>, sleep_millis: u64) -> String {
    let app_cfg_info = (s.cfg)();
    let cfg = bar_ar_bf_cfg_adapter(&app_cfg_info);
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.to_owned();
    bar_core(u, v)
}

fn bar_ar_bf_cfg_adapter<'a>(app_cfg: &'a AppCfgInfo) -> BarArBfCfgInfo<'a> {
    BarArBfCfgInfo {
        u: app_cfg.y,
        v: &app_cfg.x,
    }
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot_by_hand0(
    app_cfg: fn() -> AppCfgInfo,
) -> impl Fn(u64) -> Pin<Box<dyn Future<Output = String> + Send + Sync>> + Send + Sync {
    let bar_ar_bf_s = Arc::new(BarArBfS {
        cfg: app_cfg,
        deps: (),
    });
    let f = move |sleep_millis| {
        let bar_ar_bf_s = bar_ar_bf_s.clone();
        let x = bar_ar_bf_c(bar_ar_bf_s, sleep_millis);
        let b_d: Pin<Box<dyn Future<Output = String> + Send + Sync>> = Box::pin(x);
        b_d
    };
    f
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot_by_hand(app_cfg: fn() -> AppCfgInfo) -> Box<BarArBfT> {
    let bar_ar_bf_s = Arc::new(BarArBfS {
        cfg: app_cfg,
        deps: (),
    });
    let f = move |sleep_millis| bar_ar_bf_c(bar_ar_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

// /// Returns a boxed bar_ar_bf_closure.
// pub fn bar_ar_bf_boot(app_cfg: &AppCfgInfo) -> Box<BarArBfT> {
//     cfg_deps_ar_boot(bar_ar_bf_c, bar_ar_bf_cfg_adapter, app_cfg, ())
// }

// /// Coded without use of [cfg_deps_boot_ar_lr].
// /// Returns a leaked static reference to a bar_ar_bf closure.
// /// Since bar_ar_bf has no dependencies, there is no benefit over _boot.
// pub fn bar_ar_bf_boot_lr_by_hand(app_cfg: &AppCfgInfo) -> &'static BarArBfT {
//     let cfg = bar_ar_bf_cfg_adapter(&app_cfg);
//     let bar_ar_bf_s: &BarArBfS = Box::leak(Box::new(BarArBfS { cfg, deps: () }));
//     let f = move |sleep_millis| bar_ar_bf_c(bar_ar_bf_s, sleep_millis);
//     ref_pin_async_fn(f)
// }

// /// Returns a leaked static reference to a bar_ar_bf closure.
// /// Since bar_ar_bf has no dependencies, there is no benefit over _boot.
// pub fn bar_ar_bf_boot_lr(app_cfg: &AppCfgInfo) -> &'static BarArBfT {
//     cfg_deps_ar_boot_lr(bar_ar_bf_c, bar_ar_bf_cfg_adapter, app_cfg, ())
// }
