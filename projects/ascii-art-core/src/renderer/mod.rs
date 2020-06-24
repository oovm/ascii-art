mod data;

use crate::{AsciiArt, Canvas};
pub use data::{AsciiData, AsciiSet};

impl AsciiArt {
    pub fn render(&self) -> Canvas {
        match self.pixel_aligned {
            true => self.render_grid(),
            false => self.render_mono(),
        }
    }
    fn render_grid(&self) -> Canvas {
        unimplemented!()
    }
    fn render_mono(&self) -> Canvas {
        unimplemented!()
    }
}
