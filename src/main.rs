mod binary_utils;
mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod string_utils;

use crate::load::{
    decode_password_string, parse_bitstring_to_save_data, reorder_blocks_back,
    undo_password_addition,
};
use crate::player::PlayerArgs;
use player::Player;

fn main() {
    let mut player = Player::new_with(PlayerArgs {
        name: Some("だい".to_string()),
        level: Some(30),
        ..Default::default()
    });
    player.maximize();

    println!("player name: {}", player.name);
    println!("player adjusted_status: {:?}", player.adjusted_status());
    println!("Password: {}", player.to_password_string().unwrap());

    let encoded = decode_password_string(&player.to_password_string().unwrap()).unwrap();
    println!("encoded: {:?}", encoded);

    let raw = undo_password_addition(&encoded).unwrap();
    println!("raw: {:?}", raw);

    let bit_block = reorder_blocks_back(&raw).unwrap();
    println!("bit_block: {:?}", bit_block);

    println!(
        "decode status: {:?}",
        parse_bitstring_to_save_data(&bit_block)
    );
}
