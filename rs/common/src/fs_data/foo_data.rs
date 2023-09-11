pub use foo_a_data::*;
pub use foo_ai_data::*;
pub use foo_an_data::*;
pub use foo_at_data::*;
pub use foo_aw_data::*;
pub use foo_data::*;
pub use foo_i_data::*;

mod foo_data {
    #[derive(Debug, Clone)]
    pub struct FooSflCfgInfo {
        pub a: String,
        pub b: i32,
    }
}

mod foo_i_data {
    use super::FooSflCfgInfo;

    pub type FooISflCfgInfo = FooSflCfgInfo;
}

mod foo_a_data {
    use super::FooSflCfgInfo;
    use crate::web::actix_handler::common_respond_to;
    use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
    use axum;
    use axum::response::{IntoResponse, Response};
    use serde::{Deserialize, Serialize};

    pub type FooASflCfgInfo = FooSflCfgInfo;

    #[derive(Clone, Deserialize, Debug)]
    pub struct FooAIn {
        pub sleep_millis: u64,
    }

    #[allow(unused)]
    #[derive(Serialize, Debug)]
    pub struct FooAOut {
        pub res: String,
    }

    impl Responder for FooAOut {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            common_respond_to(self)
        }
    }

    impl IntoResponse for FooAOut {
        fn into_response(self) -> Response {
            axum::Json(self).into_response()
        }
    }
}

mod foo_an_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAnSflCfgInfo = FooASflCfgInfo;

    pub type FooAnIn = FooAIn;

    pub type FooAnOut = FooAOut;
}

mod foo_ai_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAiSflCfgInfo = FooASflCfgInfo;

    pub type FooAiIn = FooAIn;

    pub type FooAiOut = FooAOut;
}

mod foo_aw_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAwSflCfgInfo = FooASflCfgInfo;

    pub type FooAwIn = FooAIn;

    pub type FooAwOut = FooAOut;
}

mod foo_at_data {
    use super::{FooAIn, FooAOut, FooASflCfgInfo};

    pub type FooAtSflCfgInfo = FooASflCfgInfo;

    pub type FooAtIn = FooAIn;

    pub type FooAtOut = FooAOut;
}
