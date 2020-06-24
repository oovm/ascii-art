#[derive(Debug, Clone)]
pub enum AsciiArtError {}

pub type Result<T> = std::result::Result<T, AsciiArtError>;
