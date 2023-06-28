use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDeps, CfgRefCellRc, Pinfn};
use common::pin_async_fn;
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfT = Pinfn<u64, String>;

async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = BAR_A_BF_CFG.get_cfg().get_cfg();

    let cfg = BAR_A_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_A_BF_CFG: CfgDeps<BarABfCfg, ()> = CfgDeps::new();

thread_local! {
    pub static BAR_A_BF_CFG_TL: CfgRefCellRc<BarABfCfgInfo> = cfg_to_thread_local(BAR_A_BF_CFG.get_cfg());
}

pub fn get_bar_a_bf_raw(cfg: BarABfCfg) -> BarABfT {
    let _ = BAR_A_BF_CFG.set_cfg_lenient(cfg);
    pin_async_fn!(bar_a_bf)
}
