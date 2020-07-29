mod errors;
pub use errors::{BrailleArtError, Result};

mod canvas;
mod renderer;

pub use canvas::BrailleCanvas;
pub use renderer::BrailleArtMode;

/// UTF8 of first (empty) braille character
pub const BRAILLE_OFFSET: u32 = 0x2800;

#[derive(Clone, Debug)]
pub struct BrailleArt {
    pub font_line: f32,
    pub font_space: f32,
    pub mode: BrailleArtMode,
}

impl Default for BrailleArt {
    fn default() -> Self {
        Self { font_line: 16.0, font_space: 16.0, mode: Default::default() }
    }
}
