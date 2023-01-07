use crate::config::app_cfg_info::get_app_configuration;

use crate::fs::cfgadapt::bar_bf_cfg_adapter::BAR_BF_CFG_ADAPTATION;
use crate::fs::cfgadapt::foo_sfl_cfg_adapter::FOO_SFL_CFG_ADAPTATION;
use crate::fwk::cfg_src::set_adaptation_origin;

pub fn initialize() {
    println!("initialize() has been called");
    let c = get_app_configuration;
    set_adaptation_origin(&FOO_SFL_CFG_ADAPTATION, c);
    set_adaptation_origin(&BAR_BF_CFG_ADAPTATION, c);
}
