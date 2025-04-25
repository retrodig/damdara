mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

pub use constants::config::Cli;
use constants::config::Mode;
use constants::item_weapon::{ARMOR_MASTER, ITEM_MASTER, SHIELD_MASTER, WEAPON_MASTER};
use constants::status::STATUS_TABLE;
use player::Player;
use std::collections::HashSet;
use utility::status_utils::{get_status_by_level, get_status_list};

pub fn run_from_args(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let views: HashSet<String> = args
        .view
        .clone()
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let mut player = Player::new_with(args.to_player_args());
    if args.option.iter().any(|opt| opt == "max") {
        player.maximize();
    }

    let category = views
        .iter()
        .find(|s| ["item", "weapon", "armor", "shield", "status"].contains(&s.as_str()))
        .map(|s| s.as_str());
    let is_list_mode = views.contains("list");
    let index = args
        .view
        .clone()
        .unwrap_or_default()
        .iter()
        .find_map(|s| s.parse::<usize>().ok());

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
            if is_list_mode {
                // todo: 強化params
                println!("{:?}", get_status_list());
            } else {
                println!("status: {:?}", get_status_by_level(player.level()));
            }
        }
        Mode::Display => match category {
            Some("item") => print_list_or_index(&ITEM_MASTER, is_list_mode, index),
            Some("weapon") => print_list_or_index(&WEAPON_MASTER, is_list_mode, index),
            Some("armor") => print_list_or_index(&ARMOR_MASTER, is_list_mode, index),
            Some("shield") => print_list_or_index(&SHIELD_MASTER, is_list_mode, index),
            Some("status") => print_list_or_index(&STATUS_TABLE, is_list_mode, index),
            None => println!("Category not specified"),
            _ => {}
        },
    }
    Ok(())
}

fn print_list_or_index<T: std::fmt::Debug>(list: &[T], is_list_mode: bool, index: Option<usize>) {
    if is_list_mode {
        println!("{:?}", list);
    } else if let Some(i) = index {
        let safe_index = i.min(list.len().saturating_sub(1));
        println!("{:?}", list[safe_index]);
    } else {
        println!("{:?}", list);
    }
}
