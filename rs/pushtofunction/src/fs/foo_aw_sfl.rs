use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAwIn, FooAwOut, FooAwSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{rc_pin_async_fn_wss, CfgRefCellId, RcPinFnWss, RefreshMode};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAwSflT = RcPinFnWss<FooAwIn, FooAwOut>;

pub type FooAwSflCfg = CfgRefCellId<FooAwSflCfgInfo>;

#[derive(Clone)]
pub struct FooAwSflDeps {
    pub bar_aw_bf: RcPinFnWss<u64, String>,
}

pub fn foo_aw_sfl_c(cfg: FooAwSflCfg, deps: FooAwSflDeps) -> FooAwSflT {
    let f = move |input: FooAwIn| {
        let c = cfg.get_cfg();
        let d = deps.clone();
        async move {
            let FooAwIn { sleep_millis } = input;
            sleep(Duration::from_millis(sleep_millis)).await;
            let a = c.a.clone();
            let b = c.b;
            let bar_res = (d.bar_aw_bf)(0).await;
            let res = foo_core(a, b, bar_res);
            FooAwOut { res }
        }
    };
    rc_pin_async_fn_wss(f)
}

fn foo_aw_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAwSflCfgInfo {
    FooAwSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_aw_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooAwSflT {
    let foo_aw_sfl_cfg = FooAwSflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_aw_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let foo_aw_sfl_deps = FooAwSflDeps {
        bar_aw_bf: fs::bar_aw_bf_boot(app_cfg, refresh_mode),
    };
    foo_aw_sfl_c(foo_aw_sfl_cfg, foo_aw_sfl_deps)
}
