mod common_test_art;

use common_test_art::common_test;
use common_test_art::{BarBfCfgTestInput, CfgTestInput, FooSflCfgTestInput};
use tokio;

mod t1 {

    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = common_test(|| CfgTestInput {
            foo: FooSflCfgTestInput {
                a: "foo_art_test1".to_owned(),
                b: 1,
            },
            bar: BarBfCfgTestInput {
                u: 11,
                v: "bar_art_test1".to_owned(),
            },
        })
        .await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_art_test1-foo, b=4, bar=(bar: u=12, v=bar_art_test1-bar-Tx.dummy() called from bar_art_bf_c)-Tx.dummy() called from foo_art_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}

mod t2 {

    use super::*;

    #[tokio::test]
    async fn test2() {
        let res = common_test(|| CfgTestInput {
            foo: FooSflCfgTestInput {
                a: "foo_art_test2".to_owned(),
                b: 2,
            },
            bar: BarBfCfgTestInput {
                u: 22,
                v: "bar_art_test2".to_owned(),
            },
        })
        .await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_art_test2-foo, b=5, bar=(bar: u=23, v=bar_art_test2-bar-Tx.dummy() called from bar_art_bf_c)-Tx.dummy() called from foo_art_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}
