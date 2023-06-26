use crate::fs::cfgadapt::{bar_bf_adapt_cfg_src, foo_sfl_adapt_cfg_src};
use crate::fwk::RefreshMode;
use common::config::get_app_configuration;

pub fn init_no_cache() {
    println!("init_no_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_adapt_cfg_src(c, RefreshMode::Refreshable);
    bar_bf_adapt_cfg_src(c, RefreshMode::Refreshable);
}
