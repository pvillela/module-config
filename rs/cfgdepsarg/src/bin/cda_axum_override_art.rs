use axum::{routing::post, Router};
use cfgdepsarg::fs::{
    bar_art_bf_c, foo_art_sfl_c, BarArtBfCfgInfo, FooArtSflCfgInfo, FooArtSflDeps,
};
use common::{
    config::{get_pool, refresh_app_configuration},
    fwk::{
        cfg_deps_art_partial_apply_free_tx_arc, cfg_deps_art_partial_apply_free_tx_box,
        fn2_arc_with_transaction, RefInto,
    },
    web::axum_handler::handler_of_pin,
};
use std::{sync::Arc, thread, time::Duration};

struct BarBfCfgTestInput {
    pub u: i32,
    pub v: String,
}

struct FooSflCfgTestInput {
    pub a: String,
    pub b: i32,
}

struct CfgTestInput {
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

#[tokio::main]
async fn main() {
    fn cfg_src() -> CfgTestInput {
        CfgTestInput {
            foo: FooSflCfgTestInput {
                a: "foo_art_test1".to_owned(),
                b: 1,
            },
            bar: BarBfCfgTestInput {
                u: 11,
                v: "bar_art_test1".to_owned(),
            },
        }
    }

    let bar_art_bf_tx = cfg_deps_art_partial_apply_free_tx_box(bar_art_bf_c, cfg_src, ());

    let foo_art_deps = Arc::new(FooArtSflDeps {
        bar_art_bf: bar_art_bf_tx,
    });

    let foo_art_sfl_tx_arc =
        cfg_deps_art_partial_apply_free_tx_arc(foo_art_sfl_c, cfg_src, foo_art_deps);
    let foo_art_sfl = fn2_arc_with_transaction(get_pool(), foo_art_sfl_tx_arc);

    let foo_art_sfl_hdlr = handler_of_pin(foo_art_sfl);

    let app = Router::new().route("/", post(foo_art_sfl_hdlr));

    let _ = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
        refresh_app_configuration();
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
