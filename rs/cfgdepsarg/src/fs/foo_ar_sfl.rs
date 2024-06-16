use super::{BarArBfCfgInfo, BarArBfT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooArIn, FooArOut};
use common::fs_util::foo_core;
use common::fwk::{box_pin_async_fn, CfgDeps, FromRef, Make, PinFn};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type FooArSflT = PinFn<FooArIn, FooArOut>;

pub struct FooArSflCfgInfo<'a> {
    pub a: &'a str,
    pub b: i32,
}

// #[derive(Clone)]
pub struct FooArSflDeps {
    pub bar_ar_bf: Box<BarArBfT>,
}

pub type FooArSflS<MACFG> = CfgDeps<MACFG, FooArSflDeps>;

impl<'a> FromRef<'a, FooArSflCfgInfo<'a>> for AppCfgInfo {
    fn from_ref(&'a self) -> FooArSflCfgInfo<'a> {
        FooArSflCfgInfo {
            a: &self.x,
            b: self.y,
        }
    }
}

pub async fn foo_ar_sfl_c<MACFG, ACFG>(
    s: impl Deref<Target = FooArSflS<MACFG>>,
    input: FooArIn,
) -> FooArOut
where
    MACFG: Make<ACFG>,
    ACFG: for<'a> FromRef<'a, FooArSflCfgInfo<'a>>,
{
    let app_cfg_info = s.cfg.make();
    let c = app_cfg_info.from_ref();
    let d = &s.deps;
    let FooArIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.to_owned();
    let b = c.b;
    let bar_res = (d.bar_ar_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooArOut { res }
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed foo_ar_sfl_closure.
pub fn foo_ar_sfl_boot_by_hand<ACFG>(
    app_cfg: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Box<FooArSflT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> FromRef<'a, FooArSflCfgInfo<'a>>,
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
{
    let deps = FooArSflDeps {
        bar_ar_bf: fs::bar_ar_bf_boot_by_hand(app_cfg.clone()),
    };
    let foo_ar_sfl_s = Arc::new(FooArSflS { cfg: app_cfg, deps });
    let f = move |input| foo_ar_sfl_c(foo_ar_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed foo_ar_sfl_closure.
pub fn foo_ar_sfl_boot_by_hand_mono(app_cfg: fn() -> AppCfgInfo) -> Box<FooArSflT> {
    let deps = FooArSflDeps {
        bar_ar_bf: fs::bar_ar_bf_boot_by_hand_mono(app_cfg),
    };
    let foo_ar_sfl_s = Arc::new(FooArSflS { cfg: app_cfg, deps });
    let f = move |input| foo_ar_sfl_c(foo_ar_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

// /// Returns a boxed foo_ar_sfl_closure.
// pub fn foo_ar_sfl_boot(app_cfg: &AppCfgInfo) -> Box<FooArSflT> {
//     let deps = FooArSflDeps {
//         bar_ar_bf: fs::bar_ar_bf_boot(app_cfg),
//     };
//     cfg_deps_ar_boot(foo_ar_sfl_c, foo_ar_sfl_cfg_adapter, app_cfg, deps)
// }

// /// Coded without use of [cfg_deps_boot_ar].
// /// Returns a leaked static reference to a foo_ar_sfl closure.
// /// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
// /// for each call to the returned function.
// pub fn foo_ar_sfl_boot_lr_by_hand(app_cfg: &AppCfgInfo) -> &'static FooArSflT {
//     let cfg = foo_ar_sfl_cfg_adapter(&app_cfg);
//     let deps = FooArSflDeps {
//         bar_ar_bf: Box::new(bar_ar_bf_boot_lr(app_cfg)),
//     };
//     let foo_ar_sfl_s: &FooArSflS = Box::leak(Box::new(FooArSflS { cfg, deps }));
//     let f = move |input| foo_ar_sfl_c(foo_ar_sfl_s, input);
//     ref_pin_async_fn(f)
// }

// /// Returns a leaked static reference to a foo_ar_sfl closure.
// /// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
// /// for each call to the returned function.
// pub fn foo_ar_sfl_boot_lr(app_cfg: &AppCfgInfo) -> &'static FooArSflT {
//     let deps = FooArSflDeps {
//         bar_ar_bf: Box::new(bar_ar_bf_boot_lr(app_cfg)),
//     };
//     cfg_deps_ar_boot_lr(foo_ar_sfl_c, foo_ar_sfl_cfg_adapter, app_cfg, deps)
// }
