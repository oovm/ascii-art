use image::{Pixel, Rgb, RgbImage};

#[derive(Debug, Copy, Clone)]
pub enum ColorDetector {
    Random = 0,
    Points = 1,
    Center = 2,
}

impl ColorDetector {
    pub fn get_color(&self, original: &RgbImage, a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Rgb<u8> {
        match self {
            ColorDetector::Random => unimplemented!(),
            ColorDetector::Points => unimplemented!(),
            ColorDetector::Center => {
                let center = ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0);
                original.get_pixel(center.0 as u32, center.1 as u32).to_rgb()
            }
        }
    }
}
