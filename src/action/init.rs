use crate::tui::{enter_tui, exit_tui};

pub fn run() {
    let mut terminal = enter_tui();
    let _ = crate::tui::init::run(&mut terminal);
    exit_tui();
}
