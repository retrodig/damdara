mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

use crate::load::decode_from_password_string;
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

    let password = player.to_password_string().unwrap();
    println!("Password: {}", password);

    let status = decode_from_password_string(&password).unwrap();
    println!("decode save data: {:?}", status);
}
