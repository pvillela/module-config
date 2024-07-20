mod common_test_artc;

use cfgdepsarg::fs::{Cfg, CfgCtx};
use common_test_artc::common_test;
use common_test_artc::{BarBfCfgTestInput, CfgTestInput, FooSflCfgTestInput};
use tokio;

mod t1 {
    use super::*;

    struct Ctx;
    struct CtxCfg;

    impl Cfg for CtxCfg {
        type Info = CfgTestInput;

        fn cfg() -> Self::Info {
            CfgTestInput {
                foo: FooSflCfgTestInput {
                    a: "foo_artc_test1".to_owned(),
                    b: 1,
                },
                bar: BarBfCfgTestInput {
                    u: 11,
                    v: "bar_artc_test1".to_owned(),
                },
            }
        }
    }

    impl CfgCtx for Ctx {
        type Cfg = CtxCfg;
    }

    #[tokio::test]
    async fn test1() {
        let res = common_test::<Ctx>().await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_artc_test1-foo, b=4, bar=(bar: u=12, v=bar_artc_test1-bar-Tx.dummy() called from bar_artc_bf_c)-Tx.dummy() called from foo_artc_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}

mod t2 {
    use super::*;

    struct Ctx;
    struct CtxCfg;

    impl Cfg for CtxCfg {
        type Info = CfgTestInput;

        fn cfg() -> Self::Info {
            CfgTestInput {
                foo: FooSflCfgTestInput {
                    a: "foo_artc_test2".to_owned(),
                    b: 2,
                },
                bar: BarBfCfgTestInput {
                    u: 22,
                    v: "bar_artc_test2".to_owned(),
                },
            }
        }
    }

    impl CfgCtx for Ctx {
        type Cfg = CtxCfg;
    }

    #[tokio::test]
    async fn test2() {
        let res = common_test::<Ctx>().await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_artc_test2-foo, b=5, bar=(bar: u=23, v=bar_artc_test2-bar-Tx.dummy() called from bar_artc_bf_c)-Tx.dummy() called from foo_artc_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}
