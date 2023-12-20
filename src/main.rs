use clap::Parser;
use crate::cli::Cli;
use crate::tree::TreeCommand;

mod cli;
mod tree;
mod menu;
mod parser;
mod config;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliCommand {
    #[command(subcommand)]
    command: TreeCommand,
}

fn main() {

    let cli  = CliCommand::parse();
    let _ = cli.command.execute();
}
