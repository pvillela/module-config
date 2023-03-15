use super::FooSflCfgInfo;
use crate::web::common_respond_to;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub type FooAwSflCfgInfo = FooSflCfgInfo;

#[derive(Clone, Deserialize)]
pub struct FooAwIn {
    pub sleep_millis: u64,
}

#[allow(unused)]
#[derive(Serialize, Debug)]
pub struct FooAwOut {
    pub res: String,
}

impl Responder for FooAwOut {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        common_respond_to(self)
    }
}
