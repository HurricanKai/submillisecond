use std::convert::Infallible;

use lunatic::serializer::Serializer;

use super::FromOwnedRequest;
use crate::core::Body;
use crate::RequestContext;

impl<M, S> FromOwnedRequest<M, S> for RequestContext<M, S>
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(req)
    }
}

impl<M, S> FromOwnedRequest<M, S> for http::Request<Body<'static>>
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(req.request)
    }
}
