mod constants;
mod growth_type;
mod player;
mod save;
mod string_utils;

use constants::status::get_status_by_level;
use growth_type::{calculate_abc, calculate_name_total};
use save::{SaveData, SaveDataArgs};

use player::Player;

fn main() {
    // let name = "ゆうてい";
    // let result = calculate_growth_type(name);
    // println!("成長タイプの数値（mod 16）: {}", result);
    //
    // if let Some(status) = get_status_by_level(17) {
    //     println!("{}", status.pretty_string());
    // } else {
    //     println!("指定されたレベルのデータが見つかりません。");
    // }

    // let base_status = get_status_by_level(13).unwrap(); // 例: レベル13
    // let total = calculate_name_total("くみちょ"); // 名前の数値合計
    // let abc = calculate_abc(total); // A/B/C算出
    // let adjusted = base_status.apply_abc_modifiers(&abc);
    // println!("{:?}", adjusted);

    // let default_save = SaveData::new();
    // let _ = default_save.cursed_check_code();
    //
    // println!(
    //     "ビット列: {:024b} = {}",
    //     default_save.encode_name_to_bits().unwrap(),
    //     default_save.encode_name_to_bits().unwrap(),
    // );

    // let save = SaveData::new_with(SaveDataArgs {
    //     name: Some("だい".to_string()),
    //     gold: Some(20777),
    //     experience: Some(56777),
    //     weapon: Some(7),
    //     ..Default::default()
    // });
    // let password = save.to_password_string().unwrap();
    // println!("ふっかつのじゅもん: {}", password);

    let player = Player::new("だい", Some(23));

    println!("{}", player.name);
    println!("{:?}", player.adjusted_status());
    println!("{}", player.to_password_string().unwrap());
}
