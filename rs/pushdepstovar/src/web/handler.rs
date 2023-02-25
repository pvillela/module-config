use std::{pin::Pin, sync::Arc};

use crate::{
    fs::{foo_a_sfl, FooAIn, FooAOut},
    fwk::{box_pin_async_fn, BoxPinFn},
};
use actix_web::{
    body::BoxBody, http::header::ContentType, web, Handler, HttpRequest, HttpResponse, Responder,
};
use futures::Future;

impl Responder for FooAOut {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        common_respond_to(self)
    }
}

// pub async fn foo_handler(info: web::Json<FooAIn>) -> FooAOut {
//     let input = info.into_inner();
//     foo_a_sfl(input).await
// }

pub async fn foo_handler(info: web::Json<FooAIn>) -> FooAOut {
    let input = info.into_inner();
    foo_a_sfl(input).await
}

pub fn common_respond_to<T: serde::Serialize>(t: T) -> HttpResponse<BoxBody> {
    let body = serde_json::to_string(&t).unwrap();

    // Create response and set content type
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

pub fn common_handler<S: 'static + serde::Deserialize<'static>, T: Responder + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
    // f: fn(S) -> Fut,
) -> Arc<dyn Fn(web::Json<S>) -> Fut + Send + Sync + 'static>
// BoxPinFn<web::Json<S>, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
    // {
    //     box_pin_async_fn(move |info: web::Json<S>| {
    //         let input = info.into_inner();
    //         f(input)
    //     })
    // }
{
    Arc::new(move |info: web::Json<S>| {
        let input = info.into_inner();
        f(input)
    })
}

// #[derive(Clone)]
// pub struct MyHandler<S, T: 'static> {
//     f: BoxPinFn<S, T>,
// }

// impl<S: 'static, T: Send + Sync> MyHandler<S, T> {
//     pub fn new<Fut>(f: impl Fn(S) -> Fut + 'static + Send + Sync) -> MyHandler<S, T>
//     where
//         Fut: 'static + Future<Output = T> + Send + Sync,
//     {
//         Self {
//             f: box_pin_async_fn(f),
//         }
//     }
// }

// // impl<S: Clone + 'static, T: Clone> Handler<web::Json<S>> for MyHandler<S, T> {
// //     type Output = T;
// //     type Future = Pin<Box<dyn Future<Output = T>>>;

// //     fn call(&self, args: web::Json<S>) -> Self::Future {
// //         let input = args.into_inner();
// //         (self.f)(input)
// //     }
// // }

// impl<S: Clone + 'static, T: Clone> Handler<S> for MyHandler<S, T> {
//     type Output = T;
//     type Future = Pin<Box<dyn Future<Output = T>>>;

//     fn call(&self, args: S) -> Self::Future {
//         (self.f)(args)
//     }
// }
