
use image::{DynamicImage, GenericImageView, Pixel};
use itertools::Itertools;
use std::convert::TryFrom;
use crate::{BrailleArt, BRAILLE_OFFSET};
use crate::Result;


impl BrailleArt {
    pub fn render(&self)-> Result<BrailleCanvas> {

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

            Ok(())


    }
}

pub fn region_braille<F>(x: u32, y: u32, f: F) -> u32
    where
        F: Fn((u32, u32)) -> Option<bool>,
{
    let index = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (3, 0), (3, 1)]
        .iter()
        .map(|&(dy, dx)| (y * 4 + dy, x * 2 + dx))
        .enumerate()
        .map(|(index, v)| ((f(v).unwrap_or(false) as u8) << index) as u32)
        .sum::<u32>();
    return BRAILLE_OFFSET + index;
}

#[derive(Copy, Clone, Debug)]
pub enum BrailleArtMode {
    ThresholdInverse(i32),
    Threshold(i32),
    Border(i32, i32),
}

impl Default for BrailleArtMode {
    fn default() -> Self {
        Self::Threshold(30)
    }
}

fn sub_abs(a: u8, b: u8) -> u8 {
    if a > b { a - b } else { b - a }
}

impl BrailleArtMode {
    pub fn is_on(&self, img: &DynamicImage, x: u32, y: u32) -> bool {
        if !img.in_bounds(x, y) {
            return false;
        }
        match self {
            BrailleArtMode::ThresholdInverse(threshold) => {
                *threshold <= img.get_pixel(x, y).0.iter().map(|&v| v as i32).sum::<i32>()
            }
            BrailleArtMode::Threshold(threshold) => {
                *threshold >= img.get_pixel(x, y).to_rgb().0.iter().map(|&v| v as i32).sum::<i32>()
            }
            BrailleArtMode::Border(threshold, distance) => {
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
                            .map(|(&a, &b)| sub_abs(a, b) as i32)
                            .max()
                            .unwrap_or(0);

                        df >= *threshold
                    })
            }
        }
    }
}
