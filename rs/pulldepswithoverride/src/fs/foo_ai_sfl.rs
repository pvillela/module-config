use common::{
    config::{get_app_configuration, AppCfgInfo},
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
    fwk::Pinfn,
    pin_async_fn,
};
use std::sync::OnceLock;
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

use super::bar_ai_bf;

pub type FooAiSflT = Pinfn<FooAiIn, FooAiOut>;

pub struct FooAiSflDeps {
    pub bar_ai_bf: Pinfn<u64, String>,
}

pub async fn foo_ai_sfl(input: FooAiIn) -> FooAiOut {
    let FooAiIn { sleep_millis } = input;
    let FooAiSflDeps {
        bar_ai_bf: bar_a_bf,
    } = get_deps();
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = get_cfg();

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

pub static FOO_AI_SFL_CFG: OnceLock<FooAiSflCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static FooAiSflCfgInfo {
    FOO_AI_SFL_CFG.get_or_init(|| foo_ai_sfl_cfg_adapter(&get_app_configuration()))
}

thread_local! {
    pub static FOO_AI_SFL_CFG_TL: Rc<FooAiSflCfgInfo> = Rc::new(get_cfg().clone());
}

pub static FOO_AI_SFL_DEPS: OnceLock<FooAiSflDeps> = OnceLock::new();

fn get_deps() -> &'static FooAiSflDeps {
    FOO_AI_SFL_DEPS.get_or_init(|| {
        FooAiSflDeps {
            // bar_ai_bf: || todo!(), // do this before bar_ai_bf exists
            bar_ai_bf: pin_async_fn!(bar_ai_bf), // replace above with this after bar_ai_bf has been created
        }
    })
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
