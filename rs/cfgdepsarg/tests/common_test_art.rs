use cfgdepsarg::fs::{bar_art_bf_c, foo_art_sfl_c, FooArtSflDeps};
use cfgdepsarg::fs::{BarArtBfCfgInfo, FooArtSflCfgInfo};
use common::config::get_pool;
use common::fs_data::FooArtIn;
use common::fwk::{
    cfg_deps_art_partial_apply_free_tx_arc, cfg_deps_art_partial_apply_free_tx_box,
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

impl<'a> RefInto<'a, BarArtBfCfgInfo<'a>> for CfgTestInput {
    fn ref_into(&'a self) -> BarArtBfCfgInfo<'a> {
        BarArtBfCfgInfo {
            u: self.bar.u,
            v: &self.bar.v,
        }
    }
}

impl<'a> RefInto<'a, FooArtSflCfgInfo<'a>> for CfgTestInput {
    fn ref_into(&'a self) -> FooArtSflCfgInfo<'a> {
        FooArtSflCfgInfo {
            a: &self.foo.a,
            b: self.foo.b,
        }
    }
}

pub async fn common_test(cfg_src: fn() -> CfgTestInput) -> Option<String> {
    let bar_art_bf_tx = cfg_deps_art_partial_apply_free_tx_box(bar_art_bf_c, cfg_src, ());

    let foo_art_sfl_deps = Arc::new(FooArtSflDeps {
        bar_art_bf: bar_art_bf_tx,
    });

    let foo_art_sfl_tx_arc =
        cfg_deps_art_partial_apply_free_tx_arc(foo_art_sfl_c, cfg_src, foo_art_sfl_deps);
    let foo_art_sfl = fn2_arc_with_transaction(get_pool(), foo_art_sfl_tx_arc);

    let handle = tokio::spawn(async move { foo_art_sfl(FooArtIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| format!("{:?}", x));
    println!("{:?}", res);
    res
}
