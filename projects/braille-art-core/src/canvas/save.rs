use crate::{BrailleCanvas, Result};
use std::{fs, path::Path};

impl BrailleCanvas {
    pub fn save_svg(&self, path: impl AsRef<Path>) -> Result<()> {
        let _ = path;
        unimplemented!()
    }
    pub fn save_image(&self, path: impl AsRef<Path>) -> Result<()> {
        let _ = path;
        unimplemented!()
    }
    pub fn save_text(&self, path: impl AsRef<Path>) -> Result<()> {
        fs::write(path, self.draw_text().as_bytes())?;
        Ok(())
    }
}
