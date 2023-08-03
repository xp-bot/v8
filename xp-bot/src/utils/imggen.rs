use image::{ImageBuffer, RgbImage};
use std::fs;

pub fn generate_ranking_card() -> Result<(), Box<dyn std::error::Error>> {
    if !fs::metadata("./assets").is_ok() {
        fs::create_dir("./assets")?;
    }

    let mut img: RgbImage = ImageBuffer::new(510, 408);

    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }

    img.save("./assets/test.png")?;
    
    Ok(())
}
