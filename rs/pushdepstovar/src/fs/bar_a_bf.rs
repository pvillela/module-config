use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDef, CfgRefCellRc};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

type BarABfCfg = CfgRefCellRc<BarABfCfgInfo>;

pub async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = BAR_A_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_A_BF_CFG_TL: BarABfCfg =
    BarABfCfg::new_from_once_cell_def(
        &BAR_A_BF_CFG_DEF,
    )
}

pub static BAR_A_BF_CFG_DEF: OnceCell<CfgDef<BarABfCfgInfo>> = OnceCell::new();
