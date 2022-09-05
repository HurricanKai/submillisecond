use std::convert::Infallible;

use lunatic::serializer::Serializer;

use super::FromRequest;
use crate::RequestContext;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Route(pub String);

impl<M, S> FromRequest<M, S> for Route
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_request(req: &mut RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(Route(req.uri().path().to_string()))
    }
}
