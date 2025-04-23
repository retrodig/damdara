use std::str::FromStr;

mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

pub use constants::config::Cli;
use constants::config::Mode;
use player::Player;

pub fn run_from_args(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // let config = Config::from_cli(&args);
    println!("{:?}", &args);

    let mode = Mode::from_str(&args.mode).unwrap_or_else(|_| Mode::default());
    match mode {
        Mode::Start => {
            let mut player = Player::new(&args.name);
            player.maximize();

            println!("player name: {}", player.name);
            println!("player adjusted status: {:?}", player.status());

            let password = player.to_password_string().unwrap();
            println!("Password: {}", password);

            let new_player = Player::from_password_string(&password);
            println!("new_player from Password: {:?}", new_player.unwrap());
        }
    }
    Ok(())
}
