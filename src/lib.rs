extern crate futures;
extern crate tokio_service;

use futures::{IntoFuture};
use tokio_service::Service;
use std::marker::PhantomData;

/// A service implemented by a closure.
pub struct ServiceFn<F, R> {
    f: F,
    _ty: PhantomData<fn() -> R>, // don't impose Sync on R
}

impl<F, R, S> Service for ServiceFn<F, R>
    where F: Fn(R) -> S,
          S: IntoFuture,
{
    type Request = R;
    type Response = S::Item;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        (self.f)(req).into_future()
    }
}

/// Returns a `Service` backed by the given closure.
pub fn service_fn<F, R, S>(f: F) -> ServiceFn<F, R>
    where F: Fn(R) -> S,
          S: IntoFuture,
{
    ServiceFn {
        f: f,
        _ty: PhantomData,
    }
}
