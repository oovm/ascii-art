#[derive(Debug, Clone)]
pub enum AsciiArtError {
    IOError(String)
}

pub type Result<T> = std::result::Result<T, AsciiArtError>;

impl From<std::io::Error> for AsciiArtError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}",e))
    }
}