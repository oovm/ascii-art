use super::*;
use itertools::Itertools;
use std::iter::FromIterator;

impl BrailleCanvas {
    pub fn draw_text(&self) -> String {
        self.data.iter().map(|line| String::from_iter(line)).join("\n")
    }
}

impl BrailleCanvas {
    pub fn draw_svg(&self) -> String {
        unimplemented!()
    }
}

impl BrailleCanvas {
    pub fn draw_image(&self) {
        unimplemented!()
    }
}

impl BrailleCanvas {
    pub fn draw_canvas(&self) {
        unimplemented!()
    }
}
