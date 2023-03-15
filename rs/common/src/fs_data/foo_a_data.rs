use super::FooSflCfgInfo;
use crate::web::common_respond_to;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub type FooASflCfgInfo = FooSflCfgInfo;

#[derive(Clone, Deserialize)]
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
