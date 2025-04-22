mod binary_utils;
mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod string_utils;

use player::Player;

fn main() {
    let player = Player::new("だい");

    println!("{}", player.name);
    println!("{:?}", player.adjusted_status());
    println!("{}", player.to_password_string().unwrap());
}
