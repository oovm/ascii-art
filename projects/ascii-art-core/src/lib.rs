mod detectors;
mod errors;
mod renderer;

pub use detectors::{ColorDetector, EdgeDetector};
pub use errors::{Error, LowPolyResult};
pub use rand::{rngs::StdRng, thread_rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct LowPoly {
    edge_detector: EdgeDetector,
    color_detector: ColorDetector,
    rng: StdRng,
}

impl Default for LowPoly {
    fn default() -> Self {
        Self {
            edge_detector: EdgeDetector::Random,
            color_detector: ColorDetector::ThreePoint,
            rng: StdRng::from_rng(thread_rng()).unwrap(),
        }
    }
}
