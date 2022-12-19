#[derive(Debug)]
pub enum Error {
    UnsupportedPlatform,
    CommandError,
    ParseError
}