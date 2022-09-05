use std::convert::Infallible;

use http::Method;
use lunatic::serializer::Serializer;

use super::FromRequest;
use crate::RequestContext;

impl<M, S> FromRequest<M, S> for Method
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_request(req: &mut RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(req.method().clone())
    }
}
