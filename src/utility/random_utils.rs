use crate::constants::config::BIT_8_MAX;
use rand::Rng;

pub fn random_value(n: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(0..=n)
}

pub fn generate_in_range(min: u8, max: u8) -> u8 {
    if min >= max {
        return min;
    }
    let rand = random_value(BIT_8_MAX) as u16;
    let min = min as u16;
    let max = max as u16;
    let range = max - min;
    let value = min + (rand * range / 256);
    value as u8
}

/// 1 / denominator の確率で true を返す (256bit乱数版)
pub fn random_success_by_ratio(denominator: u16) -> bool {
    if denominator == 0 {
        return true; // 0分の1は必ず成功とする
    }
    let rand_val: u16 = random_value(BIT_8_MAX) as u16;
    rand_val < (256 / denominator)
}

pub fn random_success_by_percent(percent: f64) -> bool {
    if percent <= 0.0 {
        return false;
    }
    if percent >= 100.0 {
        return true;
    }
    let rand_val: u16 = random_value(BIT_8_MAX) as u16;
    let threshold = (percent * 256.0 / 100.0).round() as u16;
    rand_val < threshold
}

pub fn get_escape_rand_max_by_monster_index(index: usize) -> u8 {
    match index {
        0..=19 => 63,
        20..=29 => 94,
        30..=34 => 127,
        35..=39 => 255,
        _ => 63,
    }
}

pub fn check_escape_success(
    player_agility: u16,
    monster_defense: u16,
    monster_rand_max: u8,
) -> bool {
    let player_random = (player_agility as u32) * (random_value(BIT_8_MAX) as u32);
    let monster_random = (monster_defense as u32) * (random_value(monster_rand_max) as u32);
    player_random >= monster_random
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_success_by_percent_50() {
        let trials = 10_000;
        let mut success_count = 0;

        for _ in 0..trials {
            if random_success_by_percent(50.0) {
                success_count += 1;
            }
        }

        let success_rate = success_count as f64 / trials as f64;
        println!("Success rate at 50%: {:.2}%", success_rate * 100.0);

        // 成功率がだいたい 45%〜55% に収まっていればOK
        assert!(
            (0.45..=0.55).contains(&success_rate),
            "Success rate out of expected range: {:.2}%",
            success_rate * 100.0
        );
    }
}
