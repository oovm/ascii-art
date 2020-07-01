use crate::{AsciiData, Rgb};

#[derive(Debug, Clone)]
pub struct AsciiCanvas {
    pub data: Vec<AsciiCanvasItem>,
}

#[derive(Debug, Clone)]
pub struct AsciiCanvasItem {
    pub x: f32,
    pub y: f32,
    pub color: Rgb<u8>,
    pub data: AsciiData,
}
