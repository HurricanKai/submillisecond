use std::convert::Infallible;

use lunatic::serializer::Serializer;

use super::FromRequest;

/// Extract the remainder of the url from a wildcard route.
///
/// # Example
///
/// ```
/// fn foo_handler(Splat(splat): Splat) {
///     // GET "/foo-bar" prints "bar"
///     println!("{splat}");
/// }
///
/// router! {
///     GET "/foo-*" => foo_handler
/// }
/// ```
pub struct Splat(pub String);

impl<M, S> FromRequest<M, S> for Splat
    where
        S: Serializer<M>,
{
    type Rejection = Infallible;

    fn from_request(req: &mut crate::RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        Ok(Splat(req.reader.uri[req.reader.cursor..].to_string()))
    }
}
