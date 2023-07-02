use common::fs_data::{BarAiBfCfgInfo, FooAiIn, FooAiSflCfgInfo};
use cfgdepsmethods::fs::FooAiSflDeps;
use cfgdepsmethods::fs::{get_bar_ai_bf_raw, get_foo_ai_sfl_raw};
use tokio;

pub async fn common_test(
    foo_ai_sfl_cfg_info: FooAiSflCfgInfo,
    bar_ai_bf_cfg_info: BarAiBfCfgInfo,
) -> Option<String> {
    let bar_ai_bf = get_bar_ai_bf_raw(bar_ai_bf_cfg_info);
    let foo_deps = FooAiSflDeps { bar_ai_bf };

    let foo_ai_sfl = get_foo_ai_sfl_raw(foo_ai_sfl_cfg_info, foo_deps);

    let handle = tokio::spawn(async move { foo_ai_sfl(FooAiIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
