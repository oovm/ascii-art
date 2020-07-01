mod data;

use crate::{canvas::AsciiCanvasItem, AsciiArt, AsciiCanvas};
pub use data::{AsciiData, AsciiSet};
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};

#[derive(Copy, Clone, Debug)]
pub enum AsciiColorMode {
    Gray = 0,
    Color = 1,
    Mask = 2,
}

impl Default for AsciiColorMode {
    fn default() -> Self {
        Self::Gray
    }
}

impl AsciiArt {
    pub fn render(&self, img: DynamicImage) -> AsciiCanvas {
        unsafe {
            match self.pixel_aligned {
                true => self.render_grid(img),
                false => self.render_mono(),
            }
        }
    }
    unsafe fn render_grid(&self, img: DynamicImage) -> AsciiCanvas {
        let w = img.width() as f32 / self.font_size;
        let h = img.height() as f32 / self.font_size;
        let color_map = img.resize_exact(w.floor() as u32, h.floor() as u32, FilterType::CatmullRom).into_rgb();
        let mut items = vec![];
        for (x, y, rgb) in color_map.enumerate_pixels() {
            let gray = rgb.to_luma();
            let data = self.char_set.nearest(*gray.0.get_unchecked(0));
            items.push(AsciiCanvasItem { x: x as f32, y: y as f32, color: rgb.to_owned(), data })
        }
        AsciiCanvas { data: items }
    }
    fn render_mono(&self) -> AsciiCanvas {
        unimplemented!()
    }
}
