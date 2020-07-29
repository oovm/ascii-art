use braille_art::BrailleArt;

#[test]
fn main() {
    let ctx = BrailleArt::default();
    let out = ctx.render_path("tests/wolfram-wolf.png").unwrap();
    out.save_text("tests/wolfram-wolf.txt").unwrap()
}
