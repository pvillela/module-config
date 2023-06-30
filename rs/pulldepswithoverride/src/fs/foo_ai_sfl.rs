use common::{
    config::{get_app_configuration, AppCfgInfo},
    fs_data::{FooAiIn, FooAiOut, FooAiSflCfgInfo},
    fs_util::foo_core,
    fwk::{CfgDeps, Pinfn},
    pin_async_fn,
};
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

use super::{bar_ai_bf, BarAiBfT, BAR_AI_BF_CFG};

pub type FooAiSflT = Pinfn<FooAiIn, FooAiOut>;

pub struct FooAiSflDeps {
    pub bar_ai_bf: BarAiBfT,
}

pub async fn foo_ai_sfl(input: FooAiIn) -> FooAiOut {
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

pub static FOO_AI_SFL_CFG_DEPS: CfgDeps<FooAiSflCfgInfo, FooAiSflDeps> = CfgDeps::lazy_init(
    || foo_ai_sfl_cfg_adapter(&get_app_configuration()),
    || {
        BAR_AI_BF_CFG.prime(); // optional, just in case we want to force up-front app initialization.
        FooAiSflDeps {
            // bar_ai_bf: || todo!(), // do this before bar_ai_bf exists
            bar_ai_bf: pin_async_fn!(bar_ai_bf), // replace above with this after bar_ai_bf has been created
        }
    },
);

thread_local! {
    pub static FOO_AI_SFL_CFG_TL: Rc<FooAiSflCfgInfo> = Rc::new(FOO_AI_SFL_CFG_DEPS.get_cfg().clone());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_ai_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAiSflCfgInfo {
    FooAiSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
