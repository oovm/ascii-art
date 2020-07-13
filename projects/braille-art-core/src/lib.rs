mod errors;
pub use errors::{BrailleArtError, Result};

mod renderer;
mod canvas;

pub use canvas::BrailleCanvas;
pub use renderer::{BrailleArtMode};

/// UTF8 of first (empty) braille character
pub const BRAILLE_OFFSET: u32 = 0x2800;

#[derive(Clone, Debug)]
pub struct BrailleArt {
    pub font_line: f32,
    pub font_space: f32,
    pub mode :BrailleArtMode
}


