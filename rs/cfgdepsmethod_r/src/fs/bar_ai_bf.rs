use common::fs_data::BarAiBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDeps, Pinfn};
use common::pin_async_fn;
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAiBfT = Pinfn<u64, String>;

async fn bar_ai_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = BAR_AI_BF_CFG.get_cfg();

    let cfg = BAR_AI_BF_CFG_TL.with(|c| c.clone());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_AI_BF_CFG: CfgDeps<BarAiBfCfgInfo, ()> = CfgDeps::new();

thread_local! {
    pub static BAR_AI_BF_CFG_TL: Rc<BarAiBfCfgInfo> = Rc::new(BAR_AI_BF_CFG.get_cfg().clone());
}

pub fn get_bar_ai_bf_raw(cfg: BarAiBfCfgInfo) -> BarAiBfT {
    let _ = BAR_AI_BF_CFG.set_cfg_lenient(cfg);
    pin_async_fn!(bar_ai_bf)
}
