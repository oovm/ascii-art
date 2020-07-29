use crate::{renderer::mode::region_braille, BrailleArt, BrailleCanvas, Result};
use image::{DynamicImage, GenericImageView};

mod mode;

use image::io::Reader;
pub use mode::BrailleArtMode;
use std::{io::Cursor, path::Path};

impl BrailleArt {
    pub fn render_path(&self, path: impl AsRef<Path>) -> Result<BrailleCanvas> {
        let img = Reader::open(path)?.decode()?;
        Ok(self.render(img))
    }
    pub fn render_bytes(&self, bytes: &[u8]) -> Result<BrailleCanvas> {
        let img = Reader::new(Cursor::new(bytes)).decode()?;
        Ok(self.render(img))
    }
    pub fn render(&self, img: DynamicImage) -> BrailleCanvas {
        let (width, height) = img.dimensions();
        let mat: Vec<Vec<bool>> =
            (0..height).map(|y| (0..width).into_iter().map(|x| self.mode.is_on(&img, x, y)).collect()).collect();
        let mut out = vec![];
        for y in 0..=height / 4 {
            let mut line = vec![];
            for x in 0..=width / 2 {
                let v = region_braille(x, y, |(y, x)| {
                    if !img.in_bounds(x, y) {
                        return None;
                    }
                    Some(mat[y as usize][x as usize])
                });
                line.push(std::char::from_u32(v).unwrap())
            }
            out.push(line)
        }
        BrailleCanvas { data: out }
    }
}
