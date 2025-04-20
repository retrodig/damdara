mod constants;
mod growth_type;

use growth_type::calculate_growth_type;

fn main() {
    let name = "ゆうてい";
    let result = calculate_growth_type(name);
    println!("成長タイプの数値（mod 16）: {}", result);
}
