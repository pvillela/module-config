mod common_test_at;

use common::fs_data::BarBfCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common_test_at::common_test;
use tokio;

mod t1 {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = common_test(
            FooSflCfgInfo {
                a: "foo_at_test1".to_owned(),
                b: 1,
            },
            BarBfCfgInfo {
                u: 11,
                v: "bar_at_test1".to_owned(),
            },
        )
        .await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_at_test1-foo, b=4, bar=(bar: u=12, v=bar_at_test1-bar-Tx.dummy() called from bar_at_bf_c)-Tx.dummy() called from foo_at_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}

mod t2 {
    use super::*;

    #[tokio::test]
    async fn test2() {
        let res = common_test(
            FooSflCfgInfo {
                a: "foo_at_test2".to_owned(),
                b: 2,
            },
            BarBfCfgInfo {
                u: 22,
                v: "bar_at_test2".to_owned(),
            },
        )
        .await;

        let expected = r#"Ok(FooAOut { res: "foo: a=foo_at_test2-foo, b=5, bar=(bar: u=23, v=bar_at_test2-bar-Tx.dummy() called from bar_at_bf_c)-Tx.dummy() called from foo_at_sfl_c" })"#;
        assert_eq!(res, Some(expected.to_owned()));
    }
}
