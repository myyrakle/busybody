use crate::command::init::ConfigOptions;

use crate::tui::{enter_tui, exit_tui};

pub fn run(_: ConfigOptions) {
    let mut terminal = enter_tui();
    crate::tui::init::run(&mut terminal).unwrap();
    exit_tui();
}
