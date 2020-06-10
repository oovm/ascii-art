use image::ImageError;

#[derive(Debug, Clone)]
pub enum Error {
    NoneError,
    IOError(String),
    ImageError(String),
}

pub type LowPolyResult<T> = std::result::Result<T, Error>;

impl From<ImageError> for Error {
    fn from(e: ImageError) -> Self {
        Self::ImageError(format!("{}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}
