use super::*;
use itertools::Itertools;

impl AsciiCanvas {
    pub fn draw_svg(&self) -> String {
        let texts = self.data.iter().map(|e| e.as_svg()).join("");
        format!(
            r#"<svg viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">{style}{text}</svg>"#,
            style = self.svg_style(),
            text = texts,
            w = self.width,
            h = self.height
        )
    }

    fn svg_style(&self) -> String {
        format!(r#"<style>.ascii-art {{ font: {px}px sans-serif; }}</style>"#, px = self.font_size)
    }
}

impl AsciiCanvasItem {
    pub fn as_svg(&self) -> String {
        format!(r#"<text x="{x}" y="{y}" class="ascii-art">{text}</text>"#, x = self.x, y = self.y, text = self.data.char)
    }
}

impl AsciiCanvas {
    pub fn draw_image(&self) {}
    pub fn draw_canvas(&self) {}
}
