use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{box_pin_async_fn, AppCfg, ArcPinFn, BoxPinFn, CfgArcSwapArc};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflT = BoxPinFn<FooAIn, FooAOut>;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub fn foo_a_sfl_c(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let f = move |input: FooAIn| {
        let c = cfg.get_cfg();
        let d = deps.clone();
        async move {
            let FooAIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = (d.bar_a_bf)(0).await;
            let res = foo_core(a, b, bar_res);
            FooAOut { res }
        }
    };
    box_pin_async_fn(f)
}

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_a_sfl_boot(app_cfg: AppCfg<AppCfgInfo>) -> FooASflT {
    let app_cfg1 = app_cfg.clone();
    let foo_a_sfl_cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_a_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );
    let foo_a_sfl_deps = FooASflDeps {
        bar_a_bf: fs::bar_a_bf_boot(app_cfg),
    };
    foo_a_sfl_c(foo_a_sfl_cfg, foo_a_sfl_deps)
}

pub fn foo_a_sfl_boot_direct(app_cfg: AppCfg<AppCfgInfo>) -> FooASflT {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_a_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );

    let bar_a_bf = fs::bar_a_bf_boot(app_cfg);

    let f = move |input: FooAIn| {
        let c = cfg.get_cfg();
        let bar_a_bf = bar_a_bf.clone();
        async move {
            let FooAIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = bar_a_bf(0).await;
            let res = foo_core(a, b, bar_res);
            FooAOut { res }
        }
    };
    box_pin_async_fn(f)
}
