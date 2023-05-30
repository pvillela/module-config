use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_global_to_thread_local, get_initialized_option, CfgArcSwapArc, CfgRefCellRc, Pinfn,
};
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfT = Pinfn<u64, String>;

pub(in crate::fs) async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate calling get_my_cfg() as an alternative to using the thread-local..
    let _ = get_my_cfg().get_cfg();

    let cfg = BAR_A_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
    pub static BAR_A_BF_CFG_TL: CfgRefCellRc<BarABfCfgInfo> = unsafe{cfg_global_to_thread_local(&BAR_A_BF_CFG)};
}

pub static mut BAR_A_BF_CFG: Option<BarABfCfg> = None;

fn get_my_cfg() -> &'static BarABfCfg {
    unsafe { get_initialized_option(&BAR_A_BF_CFG) }
}
