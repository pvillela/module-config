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

use crate::fwk::{
    box_pin_async_fn, box_pin_async_fn_wss, ref_pin_async_fn, PinFn, PinFnWss, RefreshMode,
};
use crate::fwk::{BoxPinFn, CfgDeps};
use futures::Future;
use std::ops::Deref;
use std::sync::Arc;

use super::{fn2_with_transaction, DbCfg, DbErr, Tx};

//=================
// _boot

/// Returns a boxed non-async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot<C, D, A, T, ACFG, SCFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfgdapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> Box<dyn Fn(A) -> T + Send + Sync>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_factory(app_cfg, cfgdapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(&s, input);
    Box::new(stereotype)
}

/// Returns a leaked static reference to non-async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot_lr<C, D, A, T, ACFG, SCFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfgdapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> &'static (dyn Fn(A) -> T + Send + Sync)
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_factory(app_cfg, cfgdapter, refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    Box::leak(Box::new(stereotype))
}

//=================
// _boot_i

pub fn cfg_deps_boot_i<C, D, A, T, ACFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_aidapter: fn(&ACFG) -> C,
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
    let cfg = cfg_aidapter(&app_cfg);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(&s, input);
    Box::new(stereotype)
}

pub fn cfg_deps_boot_i_lr<C, D, A, T, ACFG>(
    f_c: fn(&CfgDeps<C, D>, A) -> T,
    cfg_aidapter: fn(&ACFG) -> C,
    app_cfg: impl Deref<Target = ACFG>,
    deps: D,
) -> &'static (dyn Fn(A) -> T + Send + Sync)
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static,
{
    let cfg = cfg_aidapter(&app_cfg);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    Box::leak(Box::new(stereotype))
}

//=================
// _boot_a

/// Returns a boxed async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot_a<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> BoxPinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with refreshable configuration.
pub fn cfg_deps_boot_a_lr<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(&'static CfgDeps<C, D>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> &'static PinFn<A, T>
where
    C: Send + Sync,
    D: Send + Sync,
    T: Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}

//=================
// _boot_ai

/// Returns a boxed async stereotype instance with immutable configuration.
pub fn cfg_deps_boot_ai<C, D, A, T, FUT, ACFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_aidapter: fn(&ACFG) -> C,
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
    let cfg = cfg_aidapter(&app_cfg);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with immutable configuration.
pub fn cfg_deps_boot_ai_lr<C, D, A, T, FUT, ACFG>(
    f_c: fn(&'static CfgDeps<C, D>, A) -> FUT,
    cfg_aidapter: fn(&ACFG) -> C,
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
    let cfg = cfg_aidapter(&app_cfg);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}

//=================
// _boot_aw

/// Returns a boxed async stereotype instance without Send/Sync, with refreshable configuration.
pub fn cfg_deps_boot_aw<C, D, A, T, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
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
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn_wss(stereotype)
}

//=================
// _boot_at

/// Returns a boxed async stereotype instance with refreshable configuration,
/// for a transactional stereotype constructor, executed withint a transaction.
pub fn cfg_deps_boot_at_bound_tx<'a, C, D, A, T, APPERR, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A, &'a Tx) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> BoxPinFn<A, Result<T, APPERR>>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static + Send + Sync,
    T: 'static + Send + Sync,
    APPERR: From<DbErr> + Send + Sync + 'static,
    FUT: Future<Output = Result<T, APPERR>> + Send + Sync + 'static,
    ACFG: DbCfg,
{
    let db = ACFG::get_db(&app_cfg());
    let f_c = fn2_with_transaction(&db, f_c);
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

/// Returns a boxed async stereotype instance with refreshable configuration,
/// for a transactional stereotype constructor, with the transaction handle as a free variable.
pub fn cfg_deps_boot_at_free_tx<'a, C, D, A, T, APPERR, FUT, ACFG, SCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A, &'a Tx) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> Box<dyn Fn(A, &'a Tx) -> FUT + Send + Sync + 'a>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static + Send + Sync,
    T: 'static + Send + Sync,
    APPERR: From<DbErr> + Send + Sync + 'static,
    FUT: Future<Output = Result<T, APPERR>> + Send + Sync + 'static,
    ACFG: DbCfg,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps: deps });
    let stereotype = move |input, tx| f_c(s.clone(), input, tx);
    Box::new(stereotype)
}

/// Returns a leaked static reference to async stereotype instance with refreshable configuration,
/// for a transactional stereotype constructor, executed withint a transaction.
pub fn cfg_deps_boot_at_lr_txnl<'a, C, D, A, T, APPERR, FUT, ACFG, SCFG>(
    f_c: fn(&'static CfgDeps<C, D>, A, &'a Tx) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<ACFG>, fn(&ACFG) -> SCFG, RefreshMode) -> C,
    cfg_adapter: fn(&ACFG) -> SCFG,
    app_cfg: fn() -> Arc<ACFG>,
    refresh_mode: RefreshMode,
    deps: D,
) -> &'static PinFn<A, Result<T, APPERR>>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static + Send + Sync,
    T: 'static + Send + Sync,
    APPERR: From<DbErr> + Send + Sync + 'static,
    FUT: Future<Output = Result<T, APPERR>> + Send + Sync + 'static,
    ACFG: DbCfg,
{
    let db = ACFG::get_db(&app_cfg());
    let f_c = fn2_with_transaction(&db, f_c);
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}
