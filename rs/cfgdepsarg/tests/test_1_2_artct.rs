mod common_test_artct;

use cfgdepsarg::fs::{Cfg, CfgCtx};
use common::fwk::{DbClientDefault, DbCtx};
use common_test_artct::{common_test, BarBfCfgTestInput, CfgTestInput, FooSflCfgTestInput};
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
                    a: "foo_artct_test1".to_owned(),
                    b: 1,
                },
                bar: BarBfCfgTestInput {
                    u: 11,
                    v: "bar_artct_test1".to_owned(),
                },
            }
        }
    }

    impl CfgCtx for Ctx {
        type Cfg = CtxCfg;
    }

    struct CtxDbClient;

    impl DbClientDefault for CtxDbClient {}

    impl DbCtx for Ctx {
        type DbClient = CtxDbClient;
    }

    #[tokio::test]
    async fn test1() {
        let res = common_test::<Ctx>().await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_artct_test1-foo, b=4, bar=(bar: u=12, v=bar_artct_test1-bar-Tx.dummy() called from bar_artct_bf_c)-Tx.dummy() called from foo_artct_sfl_c" })"#;
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
                    a: "foo_artct_test2".to_owned(),
                    b: 2,
                },
                bar: BarBfCfgTestInput {
                    u: 22,
                    v: "bar_artct_test2".to_owned(),
                },
            }
        }
    }

    impl CfgCtx for Ctx {
        type Cfg = CtxCfg;
    }

    struct CtxDbClient;

    impl DbClientDefault for CtxDbClient {}

    impl DbCtx for Ctx {
        type DbClient = CtxDbClient;
    }

    #[tokio::test]
    async fn test2() {
        let res = common_test::<Ctx>().await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_artct_test2-foo, b=5, bar=(bar: u=23, v=bar_artct_test2-bar-Tx.dummy() called from bar_artct_bf_c)-Tx.dummy() called from foo_artct_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}
