mod renderer;
pub use crate::renderer::AsciiData;

#[derive(Debug, Clone)]
pub struct AsciiArt {
    pixel_aligned: bool,
}

impl Default for AsciiArt {
    fn default() -> Self {
        Self { pixel_aligned: false }
    }
}
