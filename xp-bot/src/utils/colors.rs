use serenity::utils::Color;

#[allow(dead_code)]
pub fn blue() -> Color {
    Color::new(u32::from_str_radix(&dotenv::var("BLUE").unwrap()[2..], 16).unwrap())
}

#[allow(dead_code)]
pub fn green() -> Color {
    Color::new(u32::from_str_radix(&dotenv::var("GREEN").unwrap()[2..], 16).unwrap())
}

#[allow(dead_code)]
pub fn red() -> Color {
    Color::new(u32::from_str_radix(&dotenv::var("RED").unwrap()[2..], 16).unwrap())
}

#[allow(dead_code)]
pub fn gray() -> Color {
    Color::new(u32::from_str_radix(&dotenv::var("GRAY").unwrap()[2..], 16).unwrap())
}
