use common::fs_data::BarAnBfCfgInfo;
use common::fs_data::{FooAnIn, FooAnSflCfgInfo};
use common::fwk::RefreshMode;
use pulldepswithoverride::fs::{foo_a_sfl, BAR_A_BF_CFG, FOO_A_SFL_CFG};
use tokio;

pub async fn common_test(
    foo_an_sfl_cfg_info: FooAnSflCfgInfo,
    bar_an_bf_cfg_info: BarAnBfCfgInfo,
) -> Option<String> {
    FOO_A_SFL_CFG.with(|c| {
        c.update_all(
            move || foo_an_sfl_cfg_info.clone(),
            RefreshMode::NoRefresh,
            // FooAnSflDeps {
            //     bar_an_bf: arc_pin_async_fn(bar_an_bf),
            // },
        )
    });

    BAR_A_BF_CFG.with(|c| {
        c.update_all(
            move || bar_an_bf_cfg_info.clone(),
            RefreshMode::NoRefresh,
            // (),
        )
    });

    let handle = tokio::spawn(async move { foo_a_sfl(FooAnIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
