use common::fwk::{box_pin_async_fn, ref_pin_async_fn, PinFn, RefreshMode};
use common::fwk::{BoxPinFn, CfgDeps};
use futures::Future;
use std::sync::Arc;

pub fn cfg_deps_boot_a<'a, C, D, A, T, FUT, GCFG, LCFG>(
    f_c: fn(Arc<CfgDeps<C, D>>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<GCFG>, fn(&GCFG) -> LCFG, RefreshMode) -> C,
    cfg_adapter: fn(&GCFG) -> LCFG,
    deps: impl Fn() -> D,
    app_cfg: fn() -> Arc<GCFG>,
    refresh_mode: RefreshMode,
) -> BoxPinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s = Arc::new(CfgDeps { cfg, deps: deps() });
    let stereotype = move |input| f_c(s.clone(), input);
    box_pin_async_fn(stereotype)
}

pub fn cfg_deps_boot_a_lr<'a, C, D, A, T, FUT, GCFG, LCFG>(
    f_c: fn(&'static CfgDeps<C, D>, A) -> FUT,
    cfg_factory: impl Fn(fn() -> Arc<GCFG>, fn(&GCFG) -> LCFG, RefreshMode) -> C,
    cfg_adapter: fn(&GCFG) -> LCFG,
    deps: impl Fn() -> D,
    app_cfg: fn() -> Arc<GCFG>,
    refresh_mode: RefreshMode,
) -> &'static PinFn<A, T>
where
    C: 'static + Send + Sync,
    D: 'static + Send + Sync,
    A: 'static,
    T: 'static + Send + Sync,
    FUT: Future<Output = T> + Send + Sync + 'static,
{
    let cfg = cfg_factory(app_cfg, cfg_adapter, refresh_mode);
    let s_ref_leak: &CfgDeps<C, D> = Box::leak(Box::new(CfgDeps { cfg, deps: deps() }));
    let stereotype = move |input| f_c(s_ref_leak, input);
    ref_pin_async_fn(stereotype)
}
