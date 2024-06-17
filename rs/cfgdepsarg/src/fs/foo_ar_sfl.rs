use super::{bar_ar_bf_boot_lr, BarArBfCfgInfo, BarArBfT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooArIn, FooArOut};
use common::fs_util::foo_core;
use common::fwk::{box_pin_async_fn, cfg_deps_ar_boot, cfg_deps_ar_boot_lr, Make, PinFn, RefInto};
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

impl<'a> RefInto<'a, FooArSflCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> FooArSflCfgInfo<'a> {
        FooArSflCfgInfo {
            a: &self.x,
            b: self.y,
        }
    }
}

pub async fn foo_ar_sfl_c<ACFG>(
    cfg_src: impl Make<ACFG>,
    d: impl Deref<Target = FooArSflDeps>,
    input: FooArIn,
) -> FooArOut
where
    ACFG: for<'a> RefInto<'a, FooArSflCfgInfo<'a>>,
{
    let app_cfg_info = cfg_src.make();
    let c = app_cfg_info.ref_into();
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
    cfg_src: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Box<FooArSflT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, FooArSflCfgInfo<'a>>,
    ACFG: for<'a> RefInto<'a, BarArBfCfgInfo<'a>>,
{
    let deps = Arc::new(FooArSflDeps {
        bar_ar_bf: fs::bar_ar_bf_boot_by_hand(cfg_src.clone()),
    });
    let f = move |input| foo_ar_sfl_c(cfg_src.clone(), deps.clone(), input);
    box_pin_async_fn(f)
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed foo_ar_sfl_closure.
pub fn foo_ar_sfl_boot_by_hand_mono(cfg_src: fn() -> AppCfgInfo) -> Box<FooArSflT> {
    let deps = Arc::new(FooArSflDeps {
        bar_ar_bf: fs::bar_ar_bf_boot_by_hand_mono(cfg_src),
    });
    let f = move |input| foo_ar_sfl_c(cfg_src.clone(), deps.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a boxed foo_ar_sfl_closure.
pub fn foo_ar_sfl_boot<ACFG>(
    cfg_src: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Box<FooArSflT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, FooArSflCfgInfo<'a>>,
    ACFG: for<'a> RefInto<'a, BarArBfCfgInfo<'a>>,
{
    let deps = FooArSflDeps {
        bar_ar_bf: fs::bar_ar_bf_boot_by_hand(cfg_src.clone()),
    };
    cfg_deps_ar_boot(foo_ar_sfl_c, cfg_src, deps)
}

/// Returns a leaked static reference to a foo_ar_sfl closure.
/// The only benefit of this version over _boot is that it saves an Arc clone for dependencies
/// for each call to the returned function.
pub fn foo_ar_sfl_boot_lr<ACFG>(
    cfg_src: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> &'static FooArSflT
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, FooArSflCfgInfo<'a>>,
    ACFG: for<'a> RefInto<'a, BarArBfCfgInfo<'a>>,
{
    let deps = FooArSflDeps {
        bar_ar_bf: Box::new(bar_ar_bf_boot_lr(cfg_src.clone())),
    };
    cfg_deps_ar_boot_lr(foo_ar_sfl_c, cfg_src, deps)
}
