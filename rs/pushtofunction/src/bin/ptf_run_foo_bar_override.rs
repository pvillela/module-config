use pushtofunction::config::get_app_configuration;
use pushtofunction::fs::boot::{foo_sfl_boot, BAR_BF_CFG_ADAPTER, FOO_SFL_CFG_ADAPTER};
use pushtofunction::fs::{BarBfCfgInfo, FooSflCfgInfo};
use pushtofunction::fwk::{
    nil_app_cfg, update_cfg_adapter_with_const_fn, update_cfg_adapter_with_value,
};

fn main() {
    fn foo_test_src() -> FooSflCfgInfo {
        FooSflCfgInfo {
            x: "foo".to_owned(),
        }
    }

    update_cfg_adapter_with_const_fn(&FOO_SFL_CFG_ADAPTER, foo_test_src);

    update_cfg_adapter_with_value(&BAR_BF_CFG_ADAPTER, BarBfCfgInfo { z: 99 });

    {
        let foo_sfl = foo_sfl_boot(nil_app_cfg);
        foo_sfl()
    }

    {
        let foo_sfl = foo_sfl_boot(get_app_configuration);
        foo_sfl()
    }
}
