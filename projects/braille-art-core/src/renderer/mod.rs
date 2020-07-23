use crate::{renderer::mode::region_braille, BrailleArt, BrailleCanvas, Result};
use image::{DynamicImage, GenericImageView};

mod mode;

pub use mode::BrailleArtMode;

impl BrailleArt {
    pub fn render(&self, img: DynamicImage) -> Result<BrailleCanvas> {
        let img = image::open("tests/wolfram-wolf.png")?;
        let (width, height) = img.dimensions();
        let rl = BrailleArtMode::Border(20, 1);
        let mat: Vec<Vec<bool>> = (0..height).map(|y| (0..width).into_iter().map(|x| rl.is_on(&img, x, y)).collect()).collect();

        (0..=height / 4).for_each(|y| {
            (0..=width / 2).for_each(|x| {
                let v = region_braille(x, y, |(y, x)| {
                    if !img.in_bounds(x, y) {
                        return None;
                    }
                    Some(mat[y as usize][x as usize])
                });
                let chr = std::char::from_u32(v).unwrap();
                print!("{}", chr);
            });
            println!()
        });
        Ok(BrailleCanvas {})
    }
    pub fn render_path() {
        unimplemented!()
    }
    pub fn render_bytes() {
        unimplemented!()
    }
}
