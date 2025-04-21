mod constants;
mod growth_type;
mod save;
mod string_utils;

use constants::status::get_status_by_level;
use growth_type::calculate_growth_type;

fn main() {
    let name = "ゆうてい";
    let result = calculate_growth_type(name);
    println!("成長タイプの数値（mod 16）: {}", result);

    if let Some(status) = get_status_by_level(17) {
        println!("{}", status.pretty_string());
    } else {
        println!("指定されたレベルのデータが見つかりません。");
    }
}
