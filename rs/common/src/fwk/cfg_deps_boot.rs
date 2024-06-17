//! Generic higher-order functions to create stereotype instances from stereotype construction functions.
//!
//! The type parameters are as follows:
//! - `C`: configuration object type for the stereotype; same as `SCFG` in case of immutable configuration
//! - `D`: dependencies object type
//! - `A`: stereotype input parameter type
//! - `T`: stereotype output parameter type
//! - `FUT`: Future with output `T`
//! - `ACFG`: configuration info type for the application as a whole
//! - `SCFG`: configuration info type for the stereotype; same as `C` in case of immutable configuration
//!
//! The function arguments are as follows:
//! - `f_c`: constructor function for the stereotype
//! - `cfg_factory`: function that creates a configuration object
//! - `cfg_adapter`: function that transforms the application configuration info into the stereotype's
//!    configuration info
//! - `deps`: dependencies data structure for the stereotype
//! - `app_cfg`: application config info or function that returns application config info
//! - `refresh_mode`: cache refresh specification used in case of mutable configuration

use super::{AsyncBorrowFn3b3, AsyncBorrowFn4b4, Tx};
use crate::fwk::cfg::AppCfg;
use crate::fwk::{
    box_pin_async_fn, box_pin_async_fn_wss, ref_pin_async_fn, PinFn, PinFnWss, RefreshMode,
};
use crate::fwk::{BoxPinFn, CfgDeps};
use futures::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

//=================
// _boot

/// Returns a boxed non-async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot<C, D, A, T, ACFG, SCFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfgdapter: fn(&ACFG) -> SCFG,
    app_cfg: AppCfg<ACFG>,
    deps: D,
) -> Box<dyn Fn(A) -> T + Send + Sync>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_factory(app_cfg.app_src, cfgdapter, app_cfg.refresh_mode);
    let s = CfgDeps { cfg, deps };
    let stereotype = move |input| f_c(&s, input);
    Box::new(stereotype)
}

/// Returns a leaked static reference to non-async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot_lr<C, D, A, T, ACFG, SCFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfgdapter: fn(&ACFG) -> SCFG,
    app_cfg: AppCfg<ACFG>,
    deps: D,
) -> &'static (dyn Fn(A) -> T + Send + Sync)
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_factory(app_cfg.app_src, cfgdapter, app_cfg.refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    Box::leak(Box::new(stereotype))
}

//=================
// _i_boot

pub fn cfg_deps_i_boot<C, D, A, T, ACFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_adapter: fn(&ACFG) -> C,
    app_cfg: impl Deref<Target = ACFG>,
    deps: D,
) -> Box<dyn Fn(A) -> T + Send + Sync>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
    ACFG: 'static + Send + Sync,
{
    let cfg = cfg_adapter(&app_cfg);
    let s = CfgDeps { cfg, deps };
    let stereotype = move |input| f_c(&s, input);
    Box::new(stereotype)
}

pub fn cfg_deps_i_boot_lr<C, D, A, T, ACFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_adapter: fn(&ACFG) -> C,
    app_cfg: impl Deref<Target = ACFG>,
    deps: D,
) -> &'static (dyn Fn(A) -> T + Send + Sync)
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_adapter(&app_cfg);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    Box::leak(Box::new(stereotype))
}

//=================
// _a_boot

/// Returns a boxed async stereotype instance with refreshable configuration.
pub fn cfg_deps_a_boot<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: AppCfg<ACFG>,
    deps: D,
) -> BoxPinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg.app_src, cfg_adapter, app_cfg.refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with refreshable configuration.
pub fn cfg_deps_a_boot_lr<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(&'static CfgDeps<C, D>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: AppCfg<ACFG>,
    deps: D,
) -> &'static PinFn<A, T>
where
    C: Send + Sync,
    D: Send + Sync,
    T: Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg.app_src, cfg_adapter, app_cfg.refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}

//=================
// _ai_boot

/// Returns a boxed async stereotype instance with immutable configuration.
pub fn cfg_deps_ai_boot<C, D, A, T, FUT, ACFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_adapter: fn(&ACFG) -> C,
    app_cfg: &ACFG,
    deps: D,
) -> BoxPinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_adapter(&app_cfg);
    let s = Arc::new(CfgDeps { cfg, deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with immutable configuration.
pub fn cfg_deps_ai_boot_lr<C, D, A, T, FUT, ACFG>(
    f_c: fn(&'static CfgDeps<C, D>, A) -> FUT,
    cfg_adapter: fn(&ACFG) -> C,
    app_cfg: &ACFG,
    deps: D,
) -> &'static PinFn<A, T>
where
    C: Send + Sync,
    D: Send + Sync,
    T: Send + Sync,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_adapter(&app_cfg);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}

//=================
// _ar_boot

/// Returns a boxed async stereotype instance with reference-based stereotype `CfgInfo`.
pub fn cfg_deps_ar_boot<D, A, T, FUT, MACFG>(
    f_c: fn(cfg_src: MACFG, d: Arc<D>, input: A) -> FUT,
    cfg_src: MACFG,
    deps: D,
) -> BoxPinFn<A, T>
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    MACFG: Send + Sync + Clone + 'static,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let deps = Arc::new(deps);
    let stereotype = move |input| f_c(cfg_src.clone(), deps.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with reference-based stereotype `CfgInfo`.
pub fn cfg_deps_ar_boot_lr<D, A, T, FUT, MACFG>(
    f_c: fn(cfg_src: MACFG, d: &'static D, input: A) -> FUT,
    cfg_src: MACFG,
    deps: D,
) -> &'static PinFn<A, T>
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    MACFG: Send + Sync + Clone + 'static,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let deps: &D = Box::leak(Box::new(deps));
    let stereotype = move |input| f_c(cfg_src.clone(), deps, input);
    ref_pin_async_fn(stereotype)
}

//=================
// _aw_boot

/// Returns a boxed async stereotype instance without Send/Sync, with refreshable configuration.
pub fn cfg_deps_aw_boot<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> Box<PinFnWss<A, T>>
where
    C: 'static,
    D: 'static,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + 'static,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn_wss(stereotype)
}

//=================
// _at_boot

/// Returns an async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_partial_apply_free_tx_impl<CD, A, T>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, CD, A, Tx<'a>, T> + 'static,
    s: CD,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    CD: 'static + Send + Sync + Clone,
    T: 'static + Send + Sync,
{
    move |input, tx| Box::pin(f_c(s.clone(), input, tx))
}

/// Returns an async stereotype instance with refreshable configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_boot_free_tx_impl<C, D, A, T, ACFG, SCFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<C, D>>, A, Tx<'a>, T> + 'static,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps });
    move |input, tx| Box::pin(f_c(s.clone(), input, tx))
}

/// Returns a boxed async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_partial_apply_free_tx_box<CD, A, T>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, CD, A, Tx<'a>, T> + 'static,
    s: CD,
) -> Box<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    CD: 'static + Send + Sync + Clone,
    A: 'static,
    T: 'static + Send + Sync,
{
    let stereotype = cfg_deps_at_partial_apply_free_tx_impl(f_c, s);
    Box::new(stereotype)
}

/// Returns a boxed async stereotype instance with refreshable configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_boot_free_tx_box<C, D, A, T, ACFG, SCFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<C, D>>, A, Tx<'a>, T> + 'static,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C + 'static,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> Box<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static,
    SCFG: 'static,
{
    // // Code without using cfg_deps_boot_at_free_tx_no_box:
    // let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    // let s = Arc::new(CfgDeps { cfg, deps });
    // Box::new(move |input, tx| Box::pin(f_c(s.clone(), input, tx)))

    let stereotype =
        cfg_deps_at_boot_free_tx_impl(f_c, cfg_factory, cfg_adapter, app_cfg, refresh_mode, deps);
    Box::new(stereotype)
}

/// Returns an arced async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_partial_apply_free_tx_arc<CD, A, T>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, CD, A, Tx<'a>, T> + 'static,
    s: CD,
) -> Arc<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    CD: 'static + Send + Sync + Clone,
    A: 'static,
    T: 'static + Send + Sync,
{
    let stereotype = cfg_deps_at_partial_apply_free_tx_impl(f_c, s);
    Arc::new(stereotype)
}

/// Returns an arced async stereotype instance with refreshable configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_at_boot_free_tx_arc<C, D, A, T, ACFG, SCFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<C, D>>, A, Tx<'a>, T> + 'static,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C + 'static,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> Arc<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static,
    SCFG: 'static,
{
    let stereotype =
        cfg_deps_at_boot_free_tx_impl(f_c, cfg_factory, cfg_adapter, app_cfg, refresh_mode, deps);
    Arc::new(stereotype)
}

/// Returns an async stereotype instance with refreshable configuration, leaked CfgDeps,
/// and a free transaction argument, for a transactional stereotype constructor.
fn cfg_deps_at_boot_free_tx_lr_impl<C, D, A, T, ACFG, SCFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, &'static CfgDeps<C, D>, A, Tx<'a>, T> + 'static,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
    move |input, tx| Box::pin(f_c(s_ref_leak, input, tx))
}

/// Returns a leaked reference to an async stereotype instance with refreshable configuration, leaked CfgDeps,
/// and a free transaction argument, for a transactional stereotype constructor.
pub fn cfg_deps_at_boot_free_tx_lr<C, D, A, T, ACFG, SCFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, &'static CfgDeps<C, D>, A, Tx<'a>, T> + 'static,
    cfg_factory: impl Fn(fn() -> ACFG, fn(&ACFG) -> SCFG, RefreshMode) -> C + 'static,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> ACFG,
    refresh_mode: RefreshMode,
    deps: D,
) -> &'static (dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
                 + Send
                 + Sync)
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static,
    SCFG: 'static,
{
    let stereotype = cfg_deps_at_boot_free_tx_lr_impl(
        f_c,
        cfg_factory,
        cfg_adapter,
        app_cfg,
        refresh_mode,
        deps,
    );
    Box::leak(Box::new(stereotype))
}

//=================
// _art_boot

/// Returns an async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_art_partial_apply_free_tx_impl<MACFG, D, A, T>(
    f_c: impl for<'a> AsyncBorrowFn4b4<'a, MACFG, D, A, Tx<'a>, T> + 'static,
    cfg_src: MACFG,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    MACFG: 'static + Send + Sync + Clone,
    D: 'static + Send + Sync + Clone,
    T: 'static + Send + Sync,
{
    move |input, tx| Box::pin(f_c(cfg_src.clone(), deps.clone(), input, tx))
}

/// Returns a boxed async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_art_partial_apply_free_tx_box<MACFG, D, A, T>(
    f_c: impl for<'a> AsyncBorrowFn4b4<'a, MACFG, D, A, Tx<'a>, T> + 'static,
    cfg_src: MACFG,
    deps: D,
) -> Box<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    MACFG: 'static + Send + Sync + Clone,
    D: 'static + Send + Sync + Clone,
    A: 'static,
    T: 'static + Send + Sync,
{
    let stereotype = cfg_deps_art_partial_apply_free_tx_impl(f_c, cfg_src, deps);
    Box::new(stereotype)
}

/// Returns an arced async stereotype instance with a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_art_partial_apply_free_tx_arc<MACFG, D, A, T>(
    f_c: impl for<'a> AsyncBorrowFn4b4<'a, MACFG, D, A, Tx<'a>, T> + 'static,
    cfg_src: MACFG,
    deps: D,
) -> Arc<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    MACFG: 'static + Send + Sync + Clone,
    D: 'static + Send + Sync + Clone,
    A: 'static,
    T: 'static + Send + Sync,
{
    let stereotype = cfg_deps_art_partial_apply_free_tx_impl(f_c, cfg_src, deps);
    Arc::new(stereotype)
}

/// Returns an async stereotype instance with leaked deps,
/// and a free transaction argument, for a transactional stereotype constructor.
pub fn cfg_deps_art_boot_free_tx_lr_impl<MACFG, D, A, T>(
    f_c: impl for<'a> AsyncBorrowFn4b4<'a, MACFG, &'static D, A, Tx<'a>, T> + 'static,
    cfg_src: MACFG,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    MACFG: 'static + Send + Sync + Clone,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
{
    let deps_leak: &D = Box::leak(Box::new(deps));
    move |input, tx| Box::pin(f_c(cfg_src.clone(), deps_leak, input, tx))
}

/// Returns a leaked reference to an async stereotype instance with leaked deps,
/// and a free transaction argument, for a transactional stereotype constructor.
pub fn cfg_deps_art_boot_free_tx_lr<MACFG, D, A, T>(
    f_c: impl for<'a> AsyncBorrowFn4b4<'a, MACFG, &'static D, A, Tx<'a>, T> + 'static,
    cfg_src: MACFG,
    deps: D,
) -> &'static (dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
                 + Send
                 + Sync)
where
    MACFG: 'static + Send + Sync + Clone,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
{
    let stereotype = cfg_deps_art_boot_free_tx_lr_impl(f_c, cfg_src, deps);
    Box::leak(Box::new(stereotype))
}

//=================
// _ast_boot

/// Returns an async stereotype instance with simple configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_ast_boot_free_tx_impl<D, A, T, ACFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<ACFG, D>>, A, Tx<'a>, T> + 'static,
    app_cfg: ACFG,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: Send + Sync,
{
    let s = Arc::new(CfgDeps { cfg: app_cfg, deps });
    move |input, tx| Box::pin(f_c(s.clone(), input, tx))
}

/// Returns a boxed async stereotype instance with simple configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_ast_boot_free_tx_box<D, A, T, ACFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<ACFG, D>>, A, Tx<'a>, T> + 'static,
    app_cfg: ACFG,
    deps: D,
) -> Box<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static + Send + Sync + Clone,
{
    let stereotype = cfg_deps_ast_boot_free_tx_impl(f_c, app_cfg, deps);
    Box::new(stereotype)
}

/// Returns an arced async stereotype instance with simple configuration and a free transaction argument,
/// for a transactional stereotype constructor.
pub fn cfg_deps_ast_boot_free_tx_arc<D, A, T, ACFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, Arc<CfgDeps<ACFG, D>>, A, Tx<'a>, T> + 'static,
    app_cfg: ACFG,
    deps: D,
) -> Arc<
    dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync,
>
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static + Send + Sync + Clone,
{
    let stereotype = cfg_deps_ast_boot_free_tx_impl(f_c, app_cfg, deps);
    Arc::new(stereotype)
}

/// Returns an async stereotype instance with refreshable configuration, leaked CfgDeps,
/// and a free transaction argument, for a transactional stereotype constructor.
fn cfg_deps_ast_boot_free_tx_lr_impl<D, A, T, ACFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, &'static CfgDeps<ACFG, D>, A, Tx<'a>, T> + 'static,
    app_cfg: ACFG,
    deps: D,
) -> impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static + Send + Sync,
{
    let s_ref_leak: &CfgDeps<ACFG, D> = Box::leak(Box::new(CfgDeps { cfg: app_cfg, deps }));
    move |input, tx| Box::pin(f_c(s_ref_leak, input, tx))
}

/// Returns a leaked reference to an async stereotype instance with simple configuration, leaked CfgDeps,
/// and a free transaction argument, for a transactional stereotype constructor.
pub fn cfg_deps_ast_boot_free_tx_lr<D, A, T, ACFG>(
    f_c: impl for<'a> AsyncBorrowFn3b3<'a, &'static CfgDeps<ACFG, D>, A, Tx<'a>, T> + 'static,
    app_cfg: ACFG,
    deps: D,
) -> &'static (dyn for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
                 + Send
                 + Sync)
where
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    ACFG: 'static + Send + Sync + Clone,
{
    let stereotype = cfg_deps_ast_boot_free_tx_lr_impl(f_c, app_cfg, deps);
    Box::leak(Box::new(stereotype))
}
