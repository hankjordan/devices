/// An error that could be returned when retrieving device information.
#[derive(Debug)]
pub enum Error {
    /// The platform is unsupported.
    UnsupportedPlatform,

    /// An OS issue has occurred.
    CommandError,

    /// The result returned properly but could not be parsed.
    ParseError,
}
