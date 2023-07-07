use std::sync::Arc;

use cfgdepsarg_v::fs::{bar_ai_bf_c, foo_ai_sfl_c, BarAiBfS, FooAiSflDeps, FooAiSflS};
use common::fs_data::BarAiBfCfgInfo;
use common::fs_data::{FooAiIn, FooAiSflCfgInfo};
use common::fwk::box_pin_async_fn;
use tokio;

pub async fn common_test(
    foo_ai_sfl_cfg_info: FooAiSflCfgInfo,
    bar_ai_bf_cfg_info: BarAiBfCfgInfo,
) -> Option<String> {
    let bar_ai_bf = move |sleep_millis| {
        let bar_ai_bf_cfg_info = bar_ai_bf_cfg_info.clone();
        let bar_ai_bf_s = Arc::new(BarAiBfS {
            cfg: bar_ai_bf_cfg_info,
            deps: (),
        });
        bar_ai_bf_c(bar_ai_bf_s, sleep_millis)
    };

    let foo_ai_sfl_deps = FooAiSflDeps {
        bar_ai_bf: box_pin_async_fn(bar_ai_bf),
    };
    let foo_ai_sfl_s = Arc::new(FooAiSflS {
        cfg: foo_ai_sfl_cfg_info,
        deps: foo_ai_sfl_deps,
    });

    let foo_ai_sfl = |input| foo_ai_sfl_c(foo_ai_sfl_s, input);

    let handle = tokio::spawn(async move { foo_ai_sfl(FooAiIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
