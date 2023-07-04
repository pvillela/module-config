use super::Pinfn2;
use std::pin::Pin;

pub struct Dep0<S: 'static, T> {
    pub s: &'static S,
    pub f: fn(&S) -> T,
}

impl<S: 'static, T> Dep0<S, T> {
    pub fn run(&self) -> T {
        (self.f)(self.s)
    }
}

pub struct Dep1a<S: 'static, A1, T> {
    pub s: &'static S,
    pub f: Pinfn2<&'static S, A1, T>,
}

impl<S: 'static, A1, T> Dep1a<S, A1, T> {
    pub fn run(
        &self,
        a1: A1,
    ) -> Pin<Box<dyn futures::Future<Output = T> + std::marker::Send + Sync + 'static>> {
        (self.f)(self.s, a1)
    }
}
