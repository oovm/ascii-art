use image::io::Reader;
use low_poly::{DynamicImage, LowPoly, LowPolyResult};

#[test]
fn load_image() -> LowPolyResult<()> {
    let mut renderer = LowPoly::default();
    let img = Reader::open("tests/lena.png")?.decode()?;
    renderer.render(&img)?;
    renderer.as_image()?;
    println!("{}", img.into_luma().len());
    Ok(())
}
