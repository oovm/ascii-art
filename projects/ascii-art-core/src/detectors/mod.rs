#[derive(Debug, Copy, Clone)]
pub enum EdgeDetector {
    Random,
    Sobel { anomaly: f32 },
    Canny { anomaly: f32 },
}

#[derive(Debug, Copy, Clone)]
pub enum ColorDetector {
    Random,
    ThreePoint,
    TrianglePoint,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
    c1: f32,
    c2: f32,
}

impl Triangle {
    fn distance(p1: &Triangle, p2: &Triangle) -> f32 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }
}

impl EdgeDetector {
    pub fn remove_close_points(&self) {}
}
