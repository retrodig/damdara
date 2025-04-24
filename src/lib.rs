mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

pub use constants::config::Cli;
use constants::config::Mode;
use player::Player;
use utility::status_utils::{get_status_by_level, get_status_list};

pub fn run_from_args(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut player = Player::new_with(args.to_player_args());
    if args.option.iter().any(|opt| opt == "max") {
        player.maximize();
    }

    let mode = args.mode();
    match mode {
        Mode::Start => {
            println!("player name: {}", player.name);
            println!("summary: {:?}", player.summary());
            println!("strength_status: {:?}", player.strength_status());
        }
        Mode::Save => {
            println!("password: {}", player.to_password_string()?);
        }
        Mode::Load => {
            let new_player = Player::from_password_string(&args.password)?;
            println!("new_player from Password");
            println!("player name: {}", new_player.name);
            println!("summary: {:?}", new_player.summary());
            println!("strength_status: {:?}", new_player.strength_status());
        }
        Mode::Status => {
            if args.view.iter().any(|opt| opt == "list") {
                println!("{:?}", get_status_list());
            } else {
                println!("status: {:?}", get_status_by_level(player.level()));
            }
        }
    }
    Ok(())
}
