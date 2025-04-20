mod constants;
mod growth_type;

use growth_type::calculate_growth_type;
use constants::status::get_status_by_level;

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
