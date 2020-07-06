use crate::{AsciiData, Rgb};
use std::rc::Rc;

mod draw;
mod save;

#[derive(Debug, Clone)]
pub struct AsciiCanvas {
    pub data: Vec<AsciiCanvasItem>,
    pub font_size: f32,
    pub width: f32,
    pub height:f32,
}

#[derive(Debug, Clone)]
pub struct AsciiCanvasItem {
    pub x: f32,
    pub y: f32,
    pub color: Rgb<u8>,
    pub data: Rc<AsciiData>,
}
