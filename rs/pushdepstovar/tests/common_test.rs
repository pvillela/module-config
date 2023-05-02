use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{arc_pin_async_fn, init_option, RefreshMode, Src};
use pushdepstovar::fs::{
    bar_a_bf, foo_a_sfl, BarABfCfg, FooASflCfg, FooASflDeps, BAR_A_BF_CFG, FOO_A_SFL_CFG,
    FOO_A_SFL_DEPS,
};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let bar_cfg = BarABfCfg::new(
        Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    unsafe {
        init_option(&mut BAR_A_BF_CFG, bar_cfg);
        init_option(&mut FOO_A_SFL_CFG, foo_cfg);
        init_option(
            &mut FOO_A_SFL_DEPS,
            FooASflDeps {
                bar_a_bf: arc_pin_async_fn(bar_a_bf),
            },
        );
    }

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
