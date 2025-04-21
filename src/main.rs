mod constants;
mod growth_type;
mod save;
mod string_utils;

use constants::status::get_status_by_level;
use growth_type::calculate_growth_type;
use save::SaveData;

fn main() {
    let name = "ゆうてい";
    let result = calculate_growth_type(name);
    println!("成長タイプの数値（mod 16）: {}", result);

    if let Some(status) = get_status_by_level(17) {
        println!("{}", status.pretty_string());
    } else {
        println!("指定されたレベルのデータが見つかりません。");
    }

    let default_save = SaveData::new();
    let _ = default_save.cursed_check_code();

    println!(
        "ビット列: {:024b} = {}",
        default_save.encode_name_to_bits().unwrap(),
        default_save.encode_name_to_bits().unwrap(),
    );
}
