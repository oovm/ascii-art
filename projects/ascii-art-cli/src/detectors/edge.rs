use crate::{
    detectors::{Graphics, Polygon},
    DynamicImage, Error, LowPolyResult,
};
use image::GrayImage;
use imageproc::edges::canny;
use triangulation::{Delaunay, Point};

const WHITE: [u8; 1] = [255u8; 1];

#[derive(Debug, Copy, Clone)]
pub enum PolyDetector {
    Random,
    Sobel {
        anomaly: f32,
    },
    Canny {
        anomaly: f32,
        low_threshold: f32,
        high_threshold: f32,
        /// min distance between two points
        min_distance: f32,
    },
}

impl Default for PolyDetector {
    fn default() -> Self {
        Self::Canny { anomaly: 0.1, low_threshold: 10.0, high_threshold: 15.0, min_distance: 4.0 }
    }
}

impl PolyDetector {
    pub fn detect_points(&self, img: &DynamicImage, max_points: usize) -> LowPolyResult<Vec<(f32, f32)>> {
        match self {
            PolyDetector::Random => unimplemented!(),
            PolyDetector::Sobel { .. } => unimplemented!(),
            PolyDetector::Canny { anomaly, low_threshold, high_threshold, min_distance: points_min_distance } => {
                let gray = canny(&img.clone().into_luma(), *low_threshold, *high_threshold);
                let mut points = self.points_from_gray(&gray);
                // points.shuffle(&mut rng);
                points.truncate(max_points);
                self.remove_close_points(&mut points, *points_min_distance);
                // add four conner
                let width = gray.width() as f32;
                let height = gray.height() as f32;
                points.push((0., 0.));
                points.push((width, 0.));
                points.push((0., height));
                points.push((width, height));
                // return points
                Ok(points)
            }
        }
    }

    pub fn detect_polys_and_edges(&self, points: &[(f32, f32)]) -> LowPolyResult<(Vec<Graphics<f32>>, Vec<Graphics<f32>>)> {
        match self {
            PolyDetector::Random => unimplemented!(),
            PolyDetector::Sobel { .. } => unimplemented!(),
            PolyDetector::Canny { .. } => {
                let ts = delaunay(points)?;
                for e in ts.edges() {}
                for p in ts.edges() {}
            }
        }
    }

    pub fn points_from_gray(&self, gray: &GrayImage) -> Vec<(f32, f32)> {
        let mut points = Vec::with_capacity(gray.len());
        for (x, y, p) in gray.enumerate_pixels() {
            if p.0 == WHITE {
                points.push((x as f32, y as f32));
            }
        }
        return points;
    }

    pub fn remove_close_points(&self, points: &mut Vec<(f32, f32)>, min_distance: f32) {
        if min_distance <= 0.0 {
            return;
        }
        let mut i = 0;
        while i < points.len() {
            let mut j = i + 1;
            while j < points.len() {
                let dt = unsafe { distance(points.get_unchecked(i), points.get_unchecked(j)) };
                if dt < min_distance {
                    points.remove(j);
                }
                else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
}

fn distance(a: &(f32, f32), b: &(f32, f32)) -> f32 {
    let dx = (a.0 - b.0).powi(2);
    let dy = (a.1 - b.1).powi(2);
    return (dx + dy).sqrt();
}

pub fn delaunay(points: &[(f32, f32)]) -> LowPolyResult<Delaunay> {
    let points: Vec<_> = points.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let triangulation = match Delaunay::new(&points) {
        None => return Err(Error::NoneError),
        Some(s) => s,
    };
    Ok(triangulation)
}
