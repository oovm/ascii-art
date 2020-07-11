use image::ImageError;

#[derive(Debug, Clone)]
pub enum BrailleArtError {
    NoneError,
    IOError(String),
    ImageError(String),
}

pub type Result<T> = std::result::Result<T, BrailleArtError>;

impl From<std::io::Error> for BrailleArtError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<ImageError> for BrailleArtError {
    fn from(e: ImageError) -> Self {
        match e {
            ImageError::IoError(e) => Self::IOError(format!("{}", e)),
            _ => Self::ImageError(format!("{}", e)),
        }
    }
}
