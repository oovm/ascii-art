mod color;
mod edge;

pub use color::ColorDetector;
pub use edge::PolyDetector;

#[derive(Debug, Clone)]
pub enum Graphics<T> {
    Point(T, T),
    Line(T, T, T, T),
    Triangle(T, T, T, T, T, T),
    Poly(Vec<(T, T)>),
}

impl Graphics<T> {
    pub fn get_point<T>(&self) {
        match self {

        }
    }
}