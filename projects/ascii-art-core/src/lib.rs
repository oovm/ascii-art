mod canvas;
mod errors;
mod renderer;

use crate::renderer::AsciiColorMode;
pub use crate::{
    canvas::{AsciiCanvas, AsciiCanvasItem},
    errors::{AsciiArtError, Result},
    renderer::{AsciiData, AsciiSet},
};
pub use image::{Luma, Rgb};

#[derive(Debug, Clone)]
pub struct AsciiArt {
    pixel_aligned: bool,
    font_size: f32,
    char_set: AsciiSet,
    color_mode: AsciiColorMode,
}

impl Default for AsciiArt {
    fn default() -> Self {
        Self { pixel_aligned: false, font_size: 16.0, char_set: Default::default(), color_mode: Default::default() }
    }
}
