mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

use player::Player;

fn main() {
    let mut player = Player::new("だい");
    player.maximize();

    println!("player name: {}", player.name);
    println!("player adjusted_status: {:?}", player.adjusted_status());

    let password = player.to_password_string().unwrap();
    println!("Password: {}", password);

    let new_player = Player::from_password_string(&password);
    println!("new_player from Password: {:?}", new_player.unwrap());
}
