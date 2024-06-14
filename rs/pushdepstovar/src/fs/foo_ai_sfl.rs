use crate::fs::get_bar_ai_bf_with_app_cfg;
use common::config::AppCfgInfo;
use common::{
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
    fwk::{CfgDepsS, Pinfn},
    pin_async_fn,
};
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

pub type FooAiSflT = Pinfn<FooAiIn, FooAiOut>;

pub struct FooAiSflDeps {
    pub bar_ai_bf: Pinfn<u64, String>,
}

async fn foo_ai_sfl(input: FooAiIn) -> FooAiOut {
    let FooAiIn { sleep_millis } = input;
    let FooAiSflDeps { bar_ai_bf } = FOO_AI_SFL_CFG_DEPS.get_deps();

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = FOO_AI_SFL_CFG_DEPS.get_cfg();

    let (a, b) = {
        let cfg = FOO_AI_SFL_CFG_TL.with(|c| c.clone());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar_ai_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAiOut { res }
}

static FOO_AI_SFL_CFG_DEPS: CfgDepsS<FooAiSflCfgInfo, FooAiSflDeps> = CfgDepsS::new();

thread_local! {
    pub static FOO_AI_SFL_CFG_TL: Rc<FooAiSflCfgInfo> = Rc::new(FOO_AI_SFL_CFG_DEPS.get_cfg().clone());
}

pub fn get_foo_ai_sfl_raw(cfg: FooAiSflCfgInfo, deps: FooAiSflDeps) -> FooAiSflT {
    let _ = FOO_AI_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_AI_SFL_CFG_DEPS.set_deps_lenient(deps);
    pin_async_fn!(foo_ai_sfl)
}

fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_ai_sfl_with_app_cfg(app_cfg_src: fn() -> AppCfgInfo) -> FooAiSflT {
    // A stereotype should initialize its dependencies.
    let bar_ai_bf = get_bar_ai_bf_with_app_cfg(app_cfg_src);
    let deps = FooAiSflDeps { bar_ai_bf };
    get_foo_ai_sfl_raw(foo_ai_sfl_cfg_adapter(&app_cfg_src()), deps)
}
