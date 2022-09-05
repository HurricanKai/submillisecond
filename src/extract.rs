//! Types and traits for extracting data from requests.
//!
//! Many of the types and implementations were taken from [Axum](https://crates.io/crates/axum).

use lunatic::serializer::Serializer;
pub use path::Path;
#[cfg(feature = "query")]
pub use query::Query;
pub use splat::Splat;

pub mod path;
pub mod rejection;

mod body;
mod header_map;
#[cfg(feature = "json")]
mod json;
mod method;
mod params;
#[cfg(feature = "query")]
mod query;
mod request;
mod route;
mod splat;
mod string;
mod vec;

use crate::response::IntoResponse;
use crate::RequestContext;

/// Types that can be created from a request. Also known as 'extractors'.
pub trait FromRequest<M, S>: Sized
    where
        S: Serializer<M>,
{
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    fn from_request(req: &mut RequestContext<M, S>) -> Result<Self, Self::Rejection>;
}

/// Types that can be created from an owned instance of the request. This can be
/// used to avoid unecessary clones.
pub trait FromOwnedRequest<M, S>: Sized
    where
        S: Serializer<M>,
{
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Extract from an owned instance of the request.
    /// The first extractor in handlers will use this method, and can help avoid
    /// cloning in many cases.
    fn from_owned_request(req: RequestContext<M, S>) -> Result<Self, Self::Rejection>;
}

impl<T, M, S> FromOwnedRequest<M, S> for T
    where
        T: FromRequest<M, S>,
        S: Serializer<M>,
{
    type Rejection = <T as FromRequest<M, S>>::Rejection;

    fn from_owned_request(mut req: RequestContext<M, S>) -> Result<Self, Self::Rejection> {
        T::from_request(&mut req)
    }
}