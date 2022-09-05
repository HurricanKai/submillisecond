use std::convert::Infallible;

use lunatic::serializer::Serializer;

use super::FromOwnedRequest;
use crate::RequestContext;

impl<M, S> FromOwnedRequest<M, S> for Vec<u8>
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(Vec::from(req.request.into_body().as_slice()))
    }
}
