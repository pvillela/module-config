use common::fwk::CfgDeps;
use common::fwk::{ref_pin_async_fn, PinFn, RefreshMode};
use futures::Future;
use std::{ops::Deref, sync::Arc};

pub fn cfg_deps_boot<C: 'static, D: 'static, A: 'static, T: 'static, GCFG>(
    f: fn(&CfgDeps<C, D>, A) -> T,
    app_cfg: fn() -> Arc<GCFG>,
    cfg_factory: impl Fn(fn() -> Arc<GCFG>) -> C,
    deps: D,
) -> Box<dyn Fn(A) -> T> {
    let cfg = cfg_factory(app_cfg);
    let s = CfgDeps { cfg, deps };
    let f = move |a| f(&s, a);
    Box::new(f)
}

pub fn cfg_deps_boot_ia_lr<C, D, S, A, T, FUT, GCFG>(
    f: fn(&(dyn Deref<Target = CfgDeps<C, D>> + Send + Sync), A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<GCFG>) -> C,
    deps: D,
) -> impl Fn(fn() -> Arc<GCFG>) -> &'static PinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync + Copy,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    move |app_cfg| {
        let cfg = cfg_factory(app_cfg);
        let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps }));
        let dyn_deref_ref_leak: &(dyn Deref<Target = CfgDeps<C, D>> + Send + Sync) =
            Box::leak(Box::new(s_ref_leak));
        let stereotype = move |input| f(dyn_deref_ref_leak, input);
        ref_pin_async_fn(stereotype)
    }
}

pub fn cfg_deps_boot_a_lr<C, D, A, T, FUT, GCFG, LCFG>(
    f_c: fn(&(dyn Deref<Target = CfgDeps<C, D>> + Send + Sync), A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<GCFG>, fn(&GCFG) -> LCFG, RefreshMode) -> C,
    cfg_adapter: fn(&GCFG) -> LCFG,
    deps: fn() -> D,
) -> impl Fn(fn() -> Arc<GCFG>, RefreshMode) -> &'static PinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    move |app_cfg, refresh_mode| {
        let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
        let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps() }));
        let dyn_deref_ref_leak: &(dyn Deref<Target = CfgDeps<C, D>> + Send + Sync) =
            Box::leak(Box::new(s_ref_leak));
        let stereotype = move |input| f_c(dyn_deref_ref_leak, input);
        ref_pin_async_fn(stereotype)
    }
}
