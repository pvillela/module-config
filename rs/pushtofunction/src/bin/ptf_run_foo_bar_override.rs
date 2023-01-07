use pushtofunction::config::app_cfg_info::{get_app_configuration, AppCfgInfo};
use pushtofunction::fs::bar_bf::BarBfCfgInfo;
use pushtofunction::fs::boot::bar_bf_boot::BAR_BF_CFG_ADAPTER;
use pushtofunction::fs::boot::foo_sfl_boot::{foo_sfl_boot, FOO_SFL_CFG_ADAPTER};
use pushtofunction::fs::foo_sfl::FooSflCfgInfo;
use pushtofunction::fwk::lift_to_nullary::{nil_app_cfg, update_cfg_adapter_with_fn};

fn main() {
    fn foo_test_adapter(_app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
        FooSflCfgInfo {
            x: "foo".to_owned(),
        }
    }

    update_cfg_adapter_with_fn(&FOO_SFL_CFG_ADAPTER, foo_test_adapter);

    fn bar_test_adapter(_app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
        BarBfCfgInfo { z: 99 }
    }

    update_cfg_adapter_with_fn(&BAR_BF_CFG_ADAPTER, bar_test_adapter);

    {
        let foo_sfl = foo_sfl_boot(nil_app_cfg);
        foo_sfl()
    }

    {
        let foo_sfl = foo_sfl_boot(get_app_configuration);
        foo_sfl()
    }
}
