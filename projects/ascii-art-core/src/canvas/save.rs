use std::fs;
use crate::{Result, AsciiCanvas};
use std::path::Path;

impl AsciiCanvas {
    pub fn save_svg(&self, path: impl AsRef<Path>)->Result<()> {
        fs::write(path, self.draw_svg().as_bytes())?;
        Ok(())
    }
    pub fn save_image(&self) {}
}
