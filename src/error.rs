use thiserror::Error;

/// An error that could be returned when retrieving device information.
#[derive(Error, Debug)]
pub enum Error {
    #[error("this platform is unsupported")]
    /// The platform is unsupported.
    UnsupportedPlatform,

    #[error("could not retrieve device information")]
    /// An OS issue has occurred.
    CommandError,

    #[error("could not parse device information")]
    /// The result returned properly but could not be parsed.
    ParseError,
}
