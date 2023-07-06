use crate::fs::{bar_i_bf_c, BarIBfS, BarIBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;
use std::sync::{Arc, OnceLock};

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_i_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarIBfT> {
    let cfg = bar_i_bf_cfg_adapter(&app_cfg());

    // We are forced to use Arc below instead of Rc to conform to Send/Sync type bounds
    // needed for the _r variant below.
    let bar_i_bf_s = Arc::new(BarIBfS { cfg, deps: () });

    let f = move || bar_i_bf_c(&bar_i_bf_s.clone());
    Box::new(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn bar_i_bf_boot_xr(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarIBfT> {
    static BAR_I_BF_S_X: OnceLock<BarIBfS> = OnceLock::new();
    let bar_i_bf_s = BAR_I_BF_S_X.get_or_init(|| {
        let cfg = bar_i_bf_cfg_adapter(&app_cfg());
        BarIBfS { cfg, deps: () }
    });
    let f = move || bar_i_bf_c(bar_i_bf_s);
    Box::new(f)
}
