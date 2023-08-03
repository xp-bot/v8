pub fn get_required_xp(level: i32) -> usize {
    return (5 as f32 / 2 as f32 * (-1 + 20 * level.pow(2)) as f32).round() as usize;
} 