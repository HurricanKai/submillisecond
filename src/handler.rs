use lunatic::serializer::{Bincode, Serializer};

use crate::{RequestContext, Response, response::IntoResponse, extract::{FromOwnedRequest, FromRequest}};

/// A handler is implemented for any function which takes any number of
/// [extractors](crate::extract), and returns any type that implements
/// [`IntoResponse`].
///
/// To avoid unecessary clones, the [`RequestContext`], [`http::Request`],
/// [`String`], [`Vec<u8>`], [`Params`](crate::params::Params) extractors (and
/// any other types which implement [`FromOwnedRequest`] directly) should be
/// placed as the first argument, and cannot be used together in a single
/// handler.
///
/// A maximum of 16 extractor arguments may be added for a single handler.
///
/// # Handler examples
///
/// ```
/// fn index() -> &'static str {
///     "Hello, submillisecond"
/// }
///
/// use submillisecond::extract::Path;
/// use submillisecond::http::status::FOUND;
///
/// fn headers(Path(id): Path<String>) -> (StatusCode, String) {
///     (FOUND, id)
/// }
/// ```
///
/// # Middleware example
///
/// ```
/// use submillisecond::RequestContent;
/// use submillisecond::response::Response;
///
/// fn logging_layer(req: RequestContext) -> Response {
///     println!("Incoming request start");
///     let res = req.next_handler();
///     println!("Incoming request end");
///     res
/// }
/// ```
pub trait Handler<Arg = (), Ret = (), M = (), S = Bincode>
    where
        S: Serializer<M>,
{
    /// Handles the request, returning a response.
    fn handle(&self, req: RequestContext<M, S>) -> Response;
}

impl<F, M, S, R> Handler<(), R, M, S> for F
where
    F: Fn() -> R,
    R: IntoResponse,
    S: Serializer<M>,
{
    fn handle(&self, _req: RequestContext<M, S>) -> Response {
        self().into_response()
    }
}

macro_rules! impl_handler {
    ( $arg1: ident $(, $( $args: ident ),*)? ) => {
        #[allow(unused_parens)]
        impl<F, M, S, $arg1, $( $( $args, )*)? R> Handler<($arg1$(, $( $args, )*)?), R, M, S> for F
        where
            F: Fn($arg1$(, $( $args, )*)?) -> R,
            $arg1: FromOwnedRequest<M, S>,
            $( $( $args: FromRequest<M, S>, )* )?
            R: IntoResponse,
            S: Serializer<M>,
        {

            #[allow(unused_mut, unused_variables)]
            fn handle(&self, mut req: RequestContext<M, S>) -> Response {
                paste::paste! {
                    $($(
                        let [< $args:lower >] = match <$args as FromRequest::<M, S>>::from_request(&mut req) {
                            Ok(e) => e,
                            Err(err) => return err.into_response(),
                        };
                    )*)?
                    let e1 = match <$arg1 as FromOwnedRequest::<M, S>>::from_owned_request(req) {
                        Ok(e) => e,
                        Err(err) => return err.into_response(),
                    };
                    self(e1 $(, $( [< $args:lower >] ),*)?).into_response()
                }
            }
        }
    };
}

all_the_tuples!(impl_handler);