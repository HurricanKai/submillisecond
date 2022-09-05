use std::{convert, ops};

use lunatic::Mailbox;
use lunatic::net::TcpStream;
use lunatic::serializer::{Bincode, Serializer};

use crate::core::Body;
use crate::params::Params;
use crate::reader::UriReader;
use crate::Response;

/// Wrapper for [`http::Request`] containing params and cursor.
pub struct RequestContext<M = (), S = Bincode>
    where
        S : Serializer<M>,
{
    /// The [`http::Request`] instance.
    pub request: http::Request<Body<'static>>,
    /// Params collected from the router.
    pub params: Params,
    /// The uri reader.
    pub reader: UriReader,
    /// The mailbox of the handler process,
    pub mailbox: Mailbox<M, S>,
    /// The next handler.
    ///
    /// This is useful for middleware. See [`RequestContext::next_handler`].
    pub(crate) next: Option<fn(RequestContext<M, S>) -> Response>,
    /// The TCP stream.
    #[cfg_attr(not(feature = "websocket"), allow(dead_code))]
    pub(crate) stream: TcpStream,
}

impl<M, S> RequestContext<M, S>
    where
        S: Serializer<M>,
{
    /// Creates a new instance of request context.
    pub fn new(request: http::Request<Body<'static>>, mailbox: Mailbox<M, S>, stream: TcpStream) -> Self {
        let path = request.uri().path().to_string();
        RequestContext {
            request,
            params: Params::default(),
            reader: UriReader::new(path),
            mailbox,
            next: None,
            stream,
        }
    }

    /// Call the next handler, returning the response.
    ///
    /// # Panics
    ///
    /// This function might panic if no next handler exists.
    pub fn next_handler(mut self) -> Response {
        if let Some(next) = self.next.take() {
            next(self)
        } else {
            panic!("no next handler")
        }
    }

    /// Set the next handler.
    ///
    /// This is used internally by the [`router!`](crate::router) macro.
    pub fn set_next_handler(&mut self, next: fn(RequestContext<M, S>) -> Response) {
        self.next = Some(next);
    }
}

impl<'a, M, S> convert::AsRef<http::Request<Body<'a>>> for RequestContext<M, S>
    where
        S: Serializer<M>,
{
    fn as_ref(&self) -> &http::Request<Body<'a>> {
        &self.request
    }
}

impl<M, S> ops::Deref for RequestContext<M, S>
    where
        S: Serializer<M>,
{
    type Target = http::Request<Body<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl<M, S> ops::DerefMut for RequestContext<M, S>
    where
        S: Serializer<M>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}
