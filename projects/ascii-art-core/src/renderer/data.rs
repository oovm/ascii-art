use fontdue::Font;
use image::GrayImage;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
    rc::Rc,
    str::Chars,
};

#[derive(Debug, Clone)]
pub struct AsciiSet {
    pub font_size: f32,
    pub font_space: f32,
    pub font_line: f32,
    pub images: Vec<Rc<AsciiData>>,
}

#[derive(Clone)]
pub struct AsciiData {
    pub char: char,
    pub height: usize,
    pub width: usize,
    pub image: GrayImage,
    pub mean: f32,
}

impl Default for AsciiSet {
    fn default() -> Self {
        Self { font_size: 16.0, font_space: 2.0, font_line: 19.2, images: vec![] }
    }
}

impl Debug for AsciiData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AsciiData { char, height, width, image: _, mean } => f
                .debug_struct("AsciiData")
                .field("char", char)
                .field("height", height)
                .field("width", width)
                .field("mean", mean)
                .finish(),
        }
    }
}

impl AsciiData {
    pub fn rasterize(font: &Font, c: char, px: f32) -> Self {
        assert!(px > 0.0);
        let (metrics, bitmap) = font.rasterize(c, px);
        let mean = bitmap.iter().map(|&u| u as f32).sum::<f32>() / bitmap.len() as f32;
        let grey = GrayImage::from_raw(metrics.width as u32, metrics.height as u32, bitmap).unwrap();
        Self { char: c, height: metrics.height, width: metrics.width, image: grey, mean }
    }
}

impl AsciiSet {
    pub fn load_string(&mut self, font: &Font, vec: &str) {
        self.load_chars(font, vec.chars())
    }

    pub fn load_vec(&mut self, font: &Font, vec: &[char]) {
        // TODO: Better convert method
        self.load_chars(font, String::from_iter(vec.into_iter()).chars())
    }
    pub fn load_chars(&mut self, font: &Font, vec: Chars) {
        assert!(self.font_size > 0.0);
        let set = BTreeSet::from_iter(vec);
        self.images = set
            .iter()
            .map(|c| AsciiData::rasterize(font, *c, self.font_size))
            .sorted_by(|a, b| PartialOrd::partial_cmp(&a.mean, &b.mean).unwrap_or(Ordering::Equal))
            .map(|o| Rc::new(o))
            .collect();
        if let Some(s) = font.horizontal_line_metrics(self.font_size) {
            self.font_line = s.new_line_size
        }
    }

    pub fn nearest(&self, pixel: u8) -> Rc<AsciiData> {
        assert!(!self.images.is_empty());
        let out = match self.images.len() {
            1 => unsafe { self.images.get_unchecked(0) },
            _ => {
                let mut min_delta = 255.0;
                let mut result = self.images.first().unwrap();
                for item in self.images.iter() {
                    let mid = (pixel as f32 - item.mean).abs();
                    if mid < min_delta {
                        min_delta = mid;
                        result = item
                    }
                }
                result
            }
        };
        return Rc::clone(out);
    }
}
