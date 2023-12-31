pub fn get_required_xp(level: i32) -> usize {
    return (5 as f32 / 2 as f32 * (-1 + 20 * level.pow(2)) as f32).round() as usize;
}

pub fn calculate_xp_from_voice_time(
    timestamp_then: u64,
    timestamp_now: i64,
    voice_xp: i64,
    voice_cooldown: i64,
    boost_percentage: f32,
) -> u32 {
    let time_seconds = (timestamp_now - timestamp_then as i64) / 1000;
    let voice_time = time_seconds - voice_cooldown as i64;

    if voice_time < 0 {
        return 0;
    }

    (voice_time as f32 * (voice_xp as f32 / 60 as f32)) as u32
        * (boost_percentage + 1.0).floor() as u32
}

pub fn calculate_level(xp: &u64) -> i32 {
    return ((2 as f64 * xp.to_owned() as f64 + 5 as f64).sqrt() as f32 / 10 as f32).floor() as i32;
}
