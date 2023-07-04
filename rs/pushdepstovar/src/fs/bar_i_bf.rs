use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDepsS;
use std::rc::Rc;

pub type BarIBfT = fn() -> String;

fn bar_i_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = BAR_I_BF_CFG.get_cfg();

    let cfg = BAR_I_BF_CFG_TL.with(|c| c.clone());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_I_BF_CFG: CfgDepsS<BarIBfCfgInfo, ()> = CfgDepsS::new();

thread_local! {
    pub static BAR_I_BF_CFG_TL: Rc<BarIBfCfgInfo> = Rc::new(BAR_I_BF_CFG.get_cfg().clone());
}

pub fn get_bar_i_bf_raw(cfg: BarIBfCfgInfo) -> BarIBfT {
    let _ = BAR_I_BF_CFG.set_cfg_lenient(cfg);
    bar_i_bf
}
