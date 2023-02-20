use pushtofunction::config::get_app_configuration;
use pushtofunction::fs::boot::{foo_sfl_boot, BAR_BF_CFG_INFO_OVERRIDE, FOO_SFL_CFG_INFO_OVERRIDE};
use pushtofunction::fs::{BarBfCfgInfo, FooSflCfgInfo};
use pushtofunction::fwk::nil_app_cfg;

fn main() {
    FOO_SFL_CFG_INFO_OVERRIDE
        .set(FooSflCfgInfo {
            x: "foo".to_owned(),
        })
        .unwrap();

    BAR_BF_CFG_INFO_OVERRIDE
        .set(BarBfCfgInfo { z: 99 })
        .unwrap();

    {
        let foo_sfl = foo_sfl_boot(nil_app_cfg);
        foo_sfl()
    }

    {
        let foo_sfl = foo_sfl_boot(get_app_configuration);
        foo_sfl()
    }
}
