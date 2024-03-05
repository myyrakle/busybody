mod action;
mod command;
mod config;
mod tui;
mod utils;

use clap::Parser;
use command::SubCommand;

fn main() {
    let args = command::Command::parse();

    match args.action {
        SubCommand::Init(command) => {
            action::init::run(command.value);
        }
    }
}
