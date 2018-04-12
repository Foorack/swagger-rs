//! Hyper service that adds a context to an incoming request and passes it on
//! to a wrapped service.

use std::io;
use std::marker::PhantomData;
use hyper;
use hyper::{Request, Response, Error};
use super::{Push, XSpanIdString};

/// Middleware wrapper service, that should be used as the outermost layer in a
/// stack of hyper services. Adds a context to a plain `hyper::Request` that can be
/// used by subsequent layers in the stack.
#[derive(Debug)]
pub struct MiddlewareWrapper<T, C>
where
    C: Default + Push<XSpanIdString>,
{
    inner: T,
    marker: PhantomData<C>,
}

impl<T, C> MiddlewareWrapper<T, C>
where
    C: Default + Push<XSpanIdString>,
{
    /// Create a new MiddlewareWrapper struct wrapping a value
    pub fn new(inner: T) -> Self {
        MiddlewareWrapper {
            inner,
            marker: PhantomData,
        }
    }
}

impl<T, C> hyper::server::NewService for MiddlewareWrapper<T, C>
    where
        C: Default + Push<XSpanIdString>,
        T: hyper::server::NewService<Request=(Request, C::Result), Response=Response, Error=Error>,

{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Instance = MiddlewareWrapper<T::Instance, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        self.inner.new_service().map(MiddlewareWrapper::new)
    }
}

impl<T, C> hyper::server::Service for MiddlewareWrapper<T, C>
    where
        C: Default + Push<XSpanIdString>,
        T: hyper::server::Service<Request=(Request, C::Result), Response=Response, Error=Error>,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = T::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        let x_span_id = XSpanIdString::get_or_generate(&req);
        let context = C::default().push(x_span_id);
        self.inner.call((req, context))
    }
}
