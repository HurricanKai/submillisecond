use std::convert::Infallible;

use http::HeaderMap;
use lunatic::serializer::Serializer;

use super::FromRequest;
use crate::RequestContext;

impl<M, S> FromRequest<M, S> for HeaderMap
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_request(req: &mut RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(req.headers().clone())
    }
}
