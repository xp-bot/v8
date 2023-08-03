use image::{ImageBuffer, RgbImage};

pub fn generate_ranking_card() {
    let mut img: RgbImage = ImageBuffer::new(510, 408);

    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }

    img.save("./assets/test.png").unwrap();
}