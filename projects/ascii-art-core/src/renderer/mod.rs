use crate::AsciiArt;
use fontdue::Font;
use image::GrayImage;
use std::str::Chars;
use std::iter::FromIterator;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct AsciiSet {
    pub font_size: f32,
    pub img: Vec<AsciiData>
}


#[derive(Debug, Clone)]
pub struct AsciiData {
    pub height: usize,
    pub width: usize,
    pub image: GrayImage,
    pub mean: f32,
}

impl AsciiData {
    pub fn rasterize(font: &Font, c: &char, px: &f32) -> Self {
        assert!(*px>0.0);
        let (metrics, bitmap) = font.rasterize(*c, *px);
        let mean = bitmap.iter().map(|&u| u as f32).sum::<f32>()/ bitmap.len() as f32;
        let grey = GrayImage::from_raw(metrics.width as u32, metrics.height as u32, bitmap).unwrap();
        Self { height: 0, width: 0, image: grey ,mean }
    }
}


impl AsciiSet {
    pub fn build_from_string(&mut self, font: &Font, vec: &str)-> Self {
        self.build_from_chars(font, vec.chars())
    }

    pub fn build_from_vec(&mut self, font: &Font, vec: &[char])-> Self {
        // TODO: Better convert method
        self.build_from_chars(font, String::from_iter(vec.into_iter()).chars())
    }
    pub fn build_from_chars(&mut self, font: &Font, vec: Chars)-> Self {
        assert!(self.font_size > 0.0);
        let mut set = BTreeSet::from_iter(vec);
        let img = set.iter().map(|c|AsciiData::rasterize(font, c, &self.font_size)).sorted_by_key(|e|e.mean).collect_vec();
        self.img = img;

        unimplemented!()
    }
}


impl AsciiArt {
    pub fn render(&self) {}
}
