pub fn get_required_xp(level: i32) -> usize {
    return 5 / 2 * (-1 + 20 * level.pow(2)) as usize;
} 