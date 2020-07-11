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
use fontdue::Font;

#[derive(Debug, Clone)]
pub struct AsciiArt {
    pub pixel_aligned: bool,
    pub reverse_color: bool,
    pub font_size: f32,
    pub char_set: AsciiSet,
    pub color_mode: AsciiColorMode,
}

impl Default for AsciiArt {
    fn default() -> Self {
        Self {
            pixel_aligned: false,
            reverse_color: false,
            font_size: 16.0,
            char_set: Default::default(),
            color_mode: Default::default(),
        }
    }
}

pub fn get_ascii_art_set(name: &str, font: &Font) -> Option<AsciiArt> {
    let s = match name.to_lowercase().as_str() {
        "ascii" => "a",
        _ => return None
    };
    let mut ctx = AsciiArt::default();
    ctx.build_font_cache(font, s);
    Some(ctx)
}