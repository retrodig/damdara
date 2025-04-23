use clap::Parser;
use std::str::FromStr;

use crate::constants::status::Flags;
use crate::constants::text::DEFAULT_NAME;
use crate::player::PlayerArgs;
pub const DEFAULT_MODE: &str = "start";
pub const DEFAULT_MAX_PARAMETER_PASSWORD: &str = "へへみぞあうぞてえきいおくらちきこぜくゆ";

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, default_value_t = String::from(DEFAULT_NAME))]
    pub name: String,
    #[clap(short, long, default_value_t = 0)]
    pub exp: u16,
    #[clap(short, long, default_value_t = 0)]
    pub gold: u16,
    #[clap(short, long, default_value_t = 0)]
    pub weapon: u8,
    #[clap(short, long, default_value_t = 0)]
    pub armor: u8,
    #[clap(short, long, default_value_t = 0)]
    pub shield: u8,
    #[clap(short, long)]
    pub item: Vec<u8>,
    #[clap(short = 'y', long, default_value_t = 0)]
    pub herbs: u8,
    #[clap(short, long, default_value_t = 0)]
    pub keys: u8,
    #[arg(long, default_value_t = Flags::default())]
    pub flags: Flags,
    #[clap(short, long, default_value_t = String::from(DEFAULT_MODE))]
    pub mode: String,
    #[clap(
        short,
        long,
        default_value_t = String::from(DEFAULT_MAX_PARAMETER_PASSWORD)
    )]
    pub password: String,
    #[clap(short, long)]
    pub option: Vec<String>,
}

impl Cli {
    pub fn mode(&self) -> Mode {
        Mode::from_str(&self.mode).unwrap_or_else(|_| Mode::default())
    }

    pub fn to_player_args(&self) -> PlayerArgs {
        PlayerArgs {
            name: Some(self.name.clone()),
            exp: Some(self.exp),
            gold: Some(self.gold),
            weapon: Some(self.weapon),
            armor: Some(self.armor),
            shield: Some(self.shield),
            items: {
                let mut arr = [0; 8];
                for (i, item) in self.item.iter().enumerate().take(8) {
                    arr[i] = *item;
                }
                Some(arr)
            },
            herbs: Some(self.herbs),
            keys: Some(self.keys),
            flags: Some(self.flags.clone()),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Start,
    Save,
    Load,
}

impl Mode {
    pub const fn default() -> Mode {
        Mode::Start
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input.to_lowercase().as_str() {
            "start" => Ok(Mode::Start),
            "save" => Ok(Mode::Save),
            "load" => Ok(Mode::Load),
            _ => Err(()),
        }
    }
}
