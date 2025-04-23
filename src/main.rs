use clap::Parser;

mod constants;
mod growth_type;
mod load;
mod player;
mod save;
mod utility;

use damdara::Cli;

fn main() {
    let args = Cli::parse();
    damdara::run_from_args(args);
}
