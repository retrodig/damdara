use clap::Parser;
use damdara::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    damdara::run_from_args(args)
}
