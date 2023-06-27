use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarAiBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::Pinfn;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAiBfT = Pinfn<u64, String>;

pub async fn bar_ai_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg();

    let cfg = BAR_AI_BF_CFG_TL.with(|c| c.clone());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_AI_BF_CFG: OnceLock<BarAiBfCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static BarAiBfCfgInfo {
    BAR_AI_BF_CFG.get_or_init(|| bar_ai_bf_cfg_adapter(&get_app_configuration()))
}

thread_local! {
    pub static BAR_AI_BF_CFG_TL: Rc<BarAiBfCfgInfo> = Rc::new(get_cfg().clone());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
