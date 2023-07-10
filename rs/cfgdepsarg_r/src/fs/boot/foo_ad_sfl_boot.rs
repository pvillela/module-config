use super::{bar_a_bf_boot, bar_a_bf_boot_lr, cfg_deps_boot_a_lr};
use crate::fs::{
    foo_ad_sfl_c, FooASflCfg, FooASflDeps, FooASflS, FooASflT, FooAdSflCfg, FooAdSflDeps,
    FooAdSflS, FooAdSflT,
};
use bar_a_bf_boot::bar_a_bf_boot_xs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fwk::{box_pin_async_fn, ref_pin_async_fn, CfgDeps, RefreshMode};
use futures::Future;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

fn foo_ad_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_ad_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooAdSflT> {
    let cfg = FooAdSflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_ad_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooAdSflDeps {
        bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_ad_sfl_s = Arc::new(FooAdSflS { cfg, deps });
    let f = move |input| foo_ad_sfl_c(&foo_ad_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

// // The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
// pub fn foo_ad_sfl_boot_s(
//     app_cfg: fn() -> Arc<AppCfgInfo>,
//     refresh_mode: RefreshMode,
// ) -> Box<FooASflT> {
//     static FOO_A_SFL_S: OnceLock<FooASflS> = OnceLock::new();
//     let foo_ad_sfl_s = FOO_A_SFL_S.get_or_init(|| {
//         let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
//             app_cfg,
//             foo_ad_sfl_cfg_adapter,
//             refresh_mode.clone(),
//         );
//         let deps = FooASflDeps {
//             bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode.clone()),
//         };
//         FooASflS { cfg, deps }
//     });
//     let f = move |input| foo_ad_sfl_c(&foo_ad_sfl_s.clone(), input);
//     box_pin_async_fn(f)
// }

// // The only benefit of this version over the above is that it saves an Arc clone for this and its dependencies
// // for each call to the returned function.
// pub fn foo_ad_sfl_boot_xs(
//     app_cfg: fn() -> Arc<AppCfgInfo>,
//     refresh_mode: RefreshMode,
// ) -> Box<FooASflT> {
//     static FOO_A_SFL_S_X: OnceLock<FooASflS> = OnceLock::new();
//     let foo_ad_sfl_s = FOO_A_SFL_S_X.get_or_init(|| {
//         let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
//             app_cfg,
//             foo_ad_sfl_cfg_adapter,
//             refresh_mode.clone(),
//         );
//         let deps = FooASflDeps {
//             bar_a_bf: bar_a_bf_boot_xs(app_cfg, refresh_mode.clone()),
//         };
//         FooASflS { cfg, deps }
//     });
//     let f = move |input| foo_ad_sfl_c(&foo_ad_sfl_s.clone(), input);
//     box_pin_async_fn(f)
// }

/// Returns a leaked static reference to a foo_ad_sfl closure.
/// The only benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_ad_sfl_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooASflT {
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_ad_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooAdSflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };

    // Two rounds of Box::leak are required to convert the CfgDeps into the required &dyn Deref
    // to be used as an argument to foo_ad_sfl_c in the returned closure.
    let s_ref_leak: &FooAdSflS = Box::leak(Box::new(FooAdSflS { cfg, deps })); // impl Deref
    let dyn_deref_ref_leak: &(dyn Deref<Target = FooAdSflS> + Send + Sync) =
        Box::leak(Box::new(s_ref_leak));

    let f = move |input| foo_ad_sfl_c(dyn_deref_ref_leak, input);
    ref_pin_async_fn(f)
}

pub fn foo_ad_sfl_boot_lr_1(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooASflT {
    let deps = || FooAdSflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };

    let cfg_factory = FooASflCfg::new_boxed_with_cfg_adapter;
    let maker = cfg_deps_boot_a_lr(foo_ad_sfl_c, cfg_factory, foo_ad_sfl_cfg_adapter, deps);
    maker(app_cfg, refresh_mode.clone())
}
