use crate::{detectors::Polygon, LowPoly, LowPolyResult, PolyDetector};
use image::{DynamicImage, RgbImage};
use imageproc::{
    drawing::{draw_antialiased_line_segment_mut, draw_convex_polygon_mut},
    pixelops::interpolate,
};
use triangulation::{Delaunay, Point};

/// draw cache
#[derive(Clone, Debug)]
pub struct Canvas {
    pub img: Option<RgbImage>,
    pub points: Vec<(f32, f32)>,
}

bitflags! {
    pub struct OutputType: u8 {
        const Polygons = 0b00000001;
        const Edges = 0b00000010;
        const Points = 0b00000100;
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self { img: None, points: vec![] }
    }
}

impl Canvas {
    fn create_low_poly(original: &RgbImage, points: &[(f32, f32)], triangulation: &[Polygon]) -> LowPolyResult<RgbImage> {
        let mut img = RgbImage::new(original.width(), original.height());
        unimplemented!()
        // let mut tri_buf = [Point::new(0, 0); 3];
        // for tri in triangulation {
        // let a = points[tri.0];
        // let b = points[tri.1];
        // let c = points[tri.2];
        //
        // let center = ((a.x + b.x + c.x) as u32 / 3, (a.y + b.y + c.y) as u32 / 3);
        // tri_buf[0] = Point::new(a.x as i32, a.y as i32);
        // tri_buf[1] = Point::new(b.x as i32, b.y as i32);
        // tri_buf[2] = Point::new(c.x as i32, c.y as i32);
        //
        // let mut color = original.get_pixel(center.0, center.1).to_rgb();
        //
        // draw_convex_polygon_mut(&mut img, &tri_buf, color);
        //
        // let ps = [a, b, c];
        //
        // for i in 0..3 {
        // let p1 = ps[i];
        // let p2 = ps[(i + 1) % 3];
        //
        // draw_antialiased_line_segment_mut(
        // &mut img,
        // (p1.x as i32, p1.y as i32),
        // (p2.x as i32, p2.y as i32),
        // color,
        // interpolate,
        // );
        // }
        // }
        //
        // Ok(img)
        //
    }
}

impl Canvas {
    pub fn as_svg(&self, out: OutputType) -> LowPolyResult<()> {
        if out.contains(OutputType::Polygons) {
            println!("ok")
        }
        unimplemented!()
    }
    pub fn as_image(&self, out: OutputType) -> LowPolyResult<RgbImage> {
        if out.contains(OutputType::Polygons) {
            println!("ok")
        }
        unimplemented!()
    }
    pub fn as_canvas(&self, out: OutputType) -> LowPolyResult<()> {
        if out.contains(OutputType::Polygons) {
            println!("ok")
        }
        unimplemented!()
    }

    fn svg_draw_poly(&self) {}
}
