use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{RefreshMode, Src};
use common::test_support;
use pulldepswithoverride::fs::{foo_a_sfl, BarABfCfg, FooASflCfg, BAR_A_BF_CFG, FOO_A_SFL_CFG};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    static mut FOO_CFG_INFO: Option<FooASflCfgInfo> = None;
    static mut BAR_CFG_INFO: Option<BarABfCfgInfo> = None;
    unsafe {
        FOO_CFG_INFO = Some(foo_a_sfl_cfg_info);
        BAR_CFG_INFO = Some(bar_a_bf_cfg_info);

        test_support::override_lazy(&FOO_A_SFL_CFG, || {
            let src = Src::new_boxed(move || FOO_CFG_INFO.clone().unwrap());
            FooASflCfg::new(src, RefreshMode::NoRefresh)
        });

        test_support::override_lazy(&BAR_A_BF_CFG, || {
            let src = Src::new_boxed(move || BAR_CFG_INFO.clone().unwrap());
            BarABfCfg::new(src, RefreshMode::NoRefresh)
        });
    }

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
