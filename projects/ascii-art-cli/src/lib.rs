mod detectors;
pub mod devices;
mod errors;
mod renderer;
#[macro_use]
extern crate bitflags;

pub use crate::renderer::{Canvas, OutputType};
pub use detectors::{ColorDetector, PolyDetector};
pub use errors::{Error, LowPolyResult};
pub use image::DynamicImage;
pub use rand::{rngs::StdRng, thread_rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct LowPoly {
    pub poly_detector: PolyDetector,
    pub color_detector: ColorDetector,
    pub max_points: u32,
    pub point_size: u32,
    pub edge_size: u32,
    pub output: OutputType,
    pub output_size: Option<(u32, u32)>,
    pub cache: Canvas,
    pub rng: StdRng,
}

impl Default for LowPoly {
    fn default() -> Self {
        Self {
            poly_detector: Default::default(),
            color_detector: ColorDetector::Points,
            max_points: 1000,
            point_size: 2,
            edge_size: 1,
            output: OutputType::Polygons,
            output_size: None,
            cache: Default::default(),
            rng: StdRng::from_rng(thread_rng()).unwrap(),
        }
    }
}

impl LowPoly {
    pub fn render(&mut self, img: &DynamicImage) -> LowPolyResult<()> {
        unimplemented!()
    }

    pub fn as_svg(&self) -> LowPolyResult<()> {
        self.cache.as_svg(self.output)
    }

    pub fn as_image(&self) -> LowPolyResult<()> {
        let img = self.cache.as_image(self.output)?;
    }
}
