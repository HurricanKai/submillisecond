use lunatic::serializer::Serializer;

use super::rejection::{InvalidUtf8, StringRejection};
use super::FromOwnedRequest;
use crate::RequestContext;

impl<M, S> FromOwnedRequest<M, S> for String
    where
        S: Serializer<M>,
{
    type Rejection = StringRejection;

    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        let body =
            std::str::from_utf8(req.request.body().as_slice()).map_err(InvalidUtf8::from_err)?;
        Ok(String::from(body))
    }
}
