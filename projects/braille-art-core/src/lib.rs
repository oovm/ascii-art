mod errors;
pub use errors::{BrailleArtError, Result};
use image::{DynamicImage, GenericImageView, Pixel};
use itertools::Itertools;
use std::convert::TryFrom;

/// UTF8 of first (empty) braille character
pub const BRAILLE_OFFSET: u32 = 0x2800;

pub fn region_braille<F>(x: u32, y: u32, f: F) -> u32
where
    F: Fn((u32, u32)) -> Option<bool>,
{
    BRAILLE_OFFSET
        + [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (3, 0), (3, 1)]
            .iter()
            .map(|&(dy, dx)| (y * 4 + dy, x * 2 + dx))
            .enumerate()
            .map(|(index, v)| {
                ((f(v).unwrap_or(false) as u8) << index) as u32
            })
            .sum::<u32>()
}

#[derive(Copy, Clone,Debug)]
enum BrailleArt {
    PxThreshold(i32),
    InvertedPxThreshold(i32),
    Border(i32, i32),
}

fn absdiff(a: u8, b: u8) -> u8 {
    if a > b { a - b } else { b - a }
}

impl BrailleArt {
    fn is_on(&self, img: &DynamicImage, x: u32, y: u32) -> bool {
        if !img.in_bounds(x, y) {
            return false;
        }
        match self {
            BrailleArt::PxThreshold(threshold) => *threshold <= img.get_pixel(x, y).0.iter().map(|&v| v as i32).sum::<i32>(),
            BrailleArt::InvertedPxThreshold(threshold) => {
                *threshold >= img.get_pixel(x, y).to_rgb().0.iter().map(|&v| v as i32).sum::<i32>()
            }
            BrailleArt::Border(threshold, distance) => {
                let px = img.get_pixel(x, y);

                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .cartesian_product(1..=*distance)
                    .map(|(&(dx, dy), d)| (dx * d, dy * d))
                    .any(|(dx, dy)| {
                        let nx = u32::try_from(x as i32 + dx).unwrap_or(0);
                        let ny = u32::try_from(y as i32 + dy).unwrap_or(0);
                        if !img.in_bounds(nx, ny) {
                            return false;
                        }

                        let df = img
                            .get_pixel(nx, ny)
                            .0
                            .iter()
                            .zip(px.0.iter())
                            .map(|(&a, &b)| absdiff(a, b) as i32)
                            .max()
                            .unwrap_or(0);

                        df >= *threshold
                    })
            }
        }
    }
}

