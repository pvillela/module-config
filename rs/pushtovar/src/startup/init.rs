use crate::config::get_app_configuration;
use crate::fs::cfgadapt::{BAR_BF_CFG_ADAPTATION, FOO_SFL_CFG_ADAPTATION};
use crate::fwk::set_adaptation_origin;

pub fn initialize() {
    println!("initialize() has been called");
    let c = get_app_configuration;
    set_adaptation_origin(&FOO_SFL_CFG_ADAPTATION, c);
    set_adaptation_origin(&BAR_BF_CFG_ADAPTATION, c);
}
