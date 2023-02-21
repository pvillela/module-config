use crate::config::get_app_configuration;
use crate::fs::cfgadapt::{bar_bf_adapt_cfg_src, foo_sfl_adapt_cfg_src};
use crate::fwk::ArcCache;

pub fn initialize() {
    println!("initialize() has been called");
    let c = get_app_configuration;

    let mut foo_cache = ArcCache::EmptyCache;
    foo_sfl_adapt_cfg_src(c, &mut foo_cache);

    let mut bar_cache = ArcCache::EmptyCache;
    bar_bf_adapt_cfg_src(c, &mut bar_cache);
}
