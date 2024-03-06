mod action;
mod command;
mod config;
mod steps;
mod tui;
mod utils;

use clap::Parser;

fn main() {
    let args = command::Command::parse();

    if args.options.init {
        action::init::run();
    } else {
        action::start::run();
    }
}
