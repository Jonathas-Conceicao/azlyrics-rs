use derive_more::From;

pub(crate) type Result<K> = std::result::Result<K, Error>;

#[derive(Debug, From)]
pub(crate) enum Error {
    RequestError(awc::error::SendRequestError),
    PayloadError(awc::error::PayloadError),

    Utf8Parse(std::str::Utf8Error),
    IOError(std::io::Error),

    DataError,
}
