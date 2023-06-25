use common::{
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
    fwk::{get_from_once_cell, set_once_cell, Pinfn},
    pin_async_fn,
};
use once_cell::sync::OnceCell;
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

pub type FooAiSflT = Pinfn<FooAiIn, FooAiOut>;

pub struct FooAiSflDeps {
    pub bar_ai_bf: Pinfn<u64, String>,
}

async fn foo_ai_sfl(input: FooAiIn) -> FooAiOut {
    let FooAiIn { sleep_millis } = input;
    let FooAiSflDeps {
        bar_ai_bf: bar_a_bf,
    } = get_from_once_cell(&FOO_AI_SFL_DEPS);
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instea of thread-local.
    let _ = get_from_once_cell(&FOO_AI_SFL_CFG);

    let (a, b) = {
        let cfg = FOO_AI_SFL_CFG_TL.with(|c| c.clone());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAiOut { res }
}

thread_local! {
    pub static FOO_AI_SFL_CFG_TL: Rc<FooAiSflCfgInfo> = Rc::new(get_from_once_cell(&FOO_AI_SFL_CFG).clone());
}

static FOO_AI_SFL_DEPS: OnceCell<FooAiSflDeps> = OnceCell::new();

static FOO_AI_SFL_CFG: OnceCell<FooAiSflCfgInfo> = OnceCell::new();

pub fn get_foo_ai_sfl_raw(cfg: FooAiSflCfgInfo, deps: FooAiSflDeps) -> FooAiSflT {
    let _ = set_once_cell(&FOO_AI_SFL_CFG, cfg);
    let _ = set_once_cell(&FOO_AI_SFL_DEPS, deps);
    pin_async_fn!(foo_ai_sfl)
}
