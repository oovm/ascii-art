use image::{DynamicImage, GenericImageView, GrayImage};
use pix::gray::Gray;
use std::ops::Range;
use image::imageops::FilterType;

fn convert_index(x: usize) -> usize {
    match x {
        3 => 6,
        4 => 3,
        5 => 4,
        6 => 5,
        _ => panic!(),
    }
}

unsafe fn image_average(image: &GrayImage, x1: u32, x2: u32, y1: u32, y2: u32) -> f32 {
    let mut sum = 0;
    for x in x1..x2 {
        for y in y1..y2 {
            sum += image.unsafe_get_pixel(x, y).0.get_unchecked(0) as f32
        }
    }
    return sum / ((x2 - x1) * (y2 - y1)) as f32;
}


#[test]
fn test() {
    let img = image::open(opts.input)?;

    let img = match &opts.size {
        ImageSize::Default => img,
        ImageSize::Sized { width, height } => {
            if *width != img.width() || *height != img.height() {
                img.resize(*width, *height, FilterType::Triangle)
            }
            else {
                img
            }
        }
    };
    let (width, height) = img.dimensions();

    let rl = opts.rule;

    let mat: Vec<Vec<bool>> = (0..height).map(|y| (0..width).into_par_iter().map(|x| rl.is_on(&img, x, y)).collect()).collect();

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
