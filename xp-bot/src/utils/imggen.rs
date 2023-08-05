use image::RgbImage;

#[allow(dead_code)]
pub async fn image_from_url(url: &str) -> Result<RgbImage, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let img: RgbImage = image::load_from_memory(&bytes)?.to_rgb8();

    Ok(img)
}
