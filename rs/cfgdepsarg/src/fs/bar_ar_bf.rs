use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    box_pin_async_fn, cfg_deps_ar_boot, cfg_deps_ar_boot_lr, CfgDeps, FromRef, Make, PinFn,
};
use futures::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

pub type BarArBfT = PinFn<u64, String>;

pub struct BarArBfCfgInfo<'a> {
    pub u: i32,
    pub v: &'a str,
}

pub type BarArBfS<ACFG> = CfgDeps<fn() -> ACFG, ()>;

impl<'a> FromRef<'a, BarArBfCfgInfo<'a>> for AppCfgInfo {
    fn from_ref(&'a self) -> BarArBfCfgInfo<'a> {
        BarArBfCfgInfo {
            u: self.y,
            v: &self.x,
        }
    }
}

pub async fn bar_ar_bf_c<ACFG>(c: impl Make<ACFG>, sleep_millis: u64) -> String
where
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
{
    let app_cfg_info = c.make();
    let cfg = app_cfg_info.from_ref();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.to_owned();
    bar_core(u, v)
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot_by_hand0<ACFG>(
    // c: fn() -> ACFG,
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> impl Fn(u64) -> Pin<Box<dyn Future<Output = String> + Send + Sync>> + Send + Sync
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
    // fn() -> ACFG: for<'a> GetCfg<'a, ACFG, BarArBfCfgInfo<'a>>,
{
    let f = move |sleep_millis| {
        // let bar_ar_bf_s = bar_ar_bf_s.clone();
        let x = bar_ar_bf_c(c.clone(), sleep_millis);
        let b_d: Pin<Box<dyn Future<Output = String> + Send + Sync>> = Box::pin(x);
        b_d
    };
    f
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot_by_hand<ACFG>(
    // c: fn() -> ACFG,
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Box<BarArBfT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
    // fn() -> ACFG: for<'a> GetCfg<'a, ACFG, BarArBfCfgInfo<'a>>,
{
    let f = move |sleep_millis| bar_ar_bf_c(c.clone(), sleep_millis);
    box_pin_async_fn(f)
}

/// Coded without use of [cfg_deps_boot_ar].
/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot_by_hand_mono(c: fn() -> AppCfgInfo) -> Box<BarArBfT> {
    let f = move |sleep_millis| bar_ar_bf_c(c, sleep_millis);
    box_pin_async_fn(f)
}

/// Returns a boxed bar_ar_bf_closure.
pub fn bar_ar_bf_boot<ACFG>(c: impl Make<ACFG> + Send + Sync + Clone + 'static) -> Box<BarArBfT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
{
    cfg_deps_ar_boot(|c, _, arg| bar_ar_bf_c(c, arg), c, ())
}

/// Returns a leaked static reference to a bar_ar_bf closure.
/// Since bar_ar_bf has no dependencies, there is no benefit over _boot.
pub fn bar_ar_bf_boot_lr<ACFG>(
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> &'static BarArBfT
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> FromRef<'a, BarArBfCfgInfo<'a>>,
{
    cfg_deps_ar_boot_lr(|c, _, arg| bar_ar_bf_c(c, arg), c, ())
}
