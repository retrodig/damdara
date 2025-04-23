use clap::Parser;
use std::str::FromStr;

use crate::constants::status::Flags;
use crate::constants::text::DEFAULT_NAME;
pub const DEFAULT_MODE: &str = "start";

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
    #[clap(short, long, default_value_t = String::from(DEFAULT_MODE))]
    pub mode: String,
    #[arg(long, default_value_t = Flags::default())]
    pub flags: Flags,
}

#[derive(Debug)]
pub enum Mode {
    Start,
}

impl Mode {
    pub const fn default() -> Mode {
        Mode::Start
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "start" => Ok(Mode::Start),
            _ => Err(()),
        }
    }
}
