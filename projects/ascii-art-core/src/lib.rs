mod canvas;
mod errors;
mod renderer;

pub use crate::{
    canvas::Canvas,
    errors::{AsciiArtError, Result},
    renderer::{AsciiData, AsciiSet},
};

#[derive(Debug, Clone)]
pub struct AsciiArt {
    pixel_aligned: bool,
}

impl Default for AsciiArt {
    fn default() -> Self {
        Self { pixel_aligned: false }
    }
}
