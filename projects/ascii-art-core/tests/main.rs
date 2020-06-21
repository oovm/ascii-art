use ascii_art_core::AsciiData;
use fontdue::{Font, FontSettings};
use image::{DynamicImage, ImageFormat};

#[test]
fn main() {
    // Read the font data.
    let font = include_bytes!("consola.ttf") as &[u8];
    // Parse it into the font type.
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();
    // rasterize and get the layout metrics for the letter 'g' at 17px.
    let char_data = AsciiData::rasterize(&font, '我', 50.0);
    let img = DynamicImage::ImageLuma8(char_data.image);
    // let img = DynamicImage::new_luma8(metrics.width as u32,metrics.height as u32);

    img.save_with_format("test.png", ImageFormat::Png);

    let (metrics, bitmap) = font.rasterize('a', 17.0);
    println!("{:#?}", metrics);
    println!("{:#?}", bitmap.len());
}
