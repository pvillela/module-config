use cfgdepsarg::fs::{bar_artc_bf_c, foo_artc_sfl_c, CfgSrc, FooArtcIn, FooArtcSflDeps};
use cfgdepsarg::fs::{BarArtcBfCfgInfo, FooArtcSflCfgInfo};
use common::config::get_pool;
use common::fwk::{
    cfg_deps_artc_partial_apply_free_tx_arc, cfg_deps_artc_partial_apply_free_tx_box,
    fn2_arc_with_transaction, RefInto,
};
use std::sync::Arc;
use tokio;

pub struct BarBfCfgTestInput {
    pub u: i32,
    pub v: String,
}

pub struct FooSflCfgTestInput {
    pub a: String,
    pub b: i32,
}

pub struct CfgTestInput {
    pub bar: BarBfCfgTestInput,
    pub foo: FooSflCfgTestInput,
}

impl<'a> RefInto<'a, BarArtcBfCfgInfo<'a>> for CfgTestInput {
    fn ref_into(&'a self) -> BarArtcBfCfgInfo<'a> {
        BarArtcBfCfgInfo {
            u: self.bar.u,
            v: &self.bar.v,
        }
    }
}

impl<'a> RefInto<'a, FooArtcSflCfgInfo<'a>> for CfgTestInput {
    fn ref_into(&'a self) -> FooArtcSflCfgInfo<'a> {
        FooArtcSflCfgInfo {
            a: &self.foo.a,
            b: self.foo.b,
        }
    }
}

pub async fn common_test<CTX>() -> Option<String>
where
    CTX: CfgSrc<AppCfg = CfgTestInput> + 'static,
{
    let bar_artc_bf_tx = cfg_deps_artc_partial_apply_free_tx_box(bar_artc_bf_c::<CTX, ()>, ());

    let foo_artc_sfl_deps = Arc::new(FooArtcSflDeps {
        bar_artc_bf: bar_artc_bf_tx,
    });

    let foo_artc_sfl_tx_arc =
        cfg_deps_artc_partial_apply_free_tx_arc(foo_artc_sfl_c::<CTX>, foo_artc_sfl_deps);
    let foo_artc_sfl = fn2_arc_with_transaction(get_pool(), foo_artc_sfl_tx_arc);

    let handle = tokio::spawn(async move { foo_artc_sfl(FooArtcIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| format!("{:?}", x));
    println!("{:?}", res);
    res
}
