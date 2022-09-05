use std::convert::Infallible;

use lunatic::serializer::Serializer;

use super::FromOwnedRequest;
use crate::{Body, RequestContext};

impl<M, S> FromOwnedRequest<M, S> for Body<'static>
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(*req.body())
    }
}
