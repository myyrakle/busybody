use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::io::Result;

use crate::{config, steps::linux::install_service};

use super::{exit_tui, TerminalType};

pub fn run(terminal: &mut TerminalType) -> Result<()> {
    terminal.clear()?;

    let mut step = 0;

    let config_exists = config::exists_config();

    let mut overwrite = true;
    let mut slack_app_token = String::new();
    let mut slack_channel_id = String::new();
    let mut disk_threshold = 70_u32;
    let mut install_daemon = true;

    if !config_exists {
        step = 1;
    }

    let mut stacked_text = String::new();
    let mut render_text = String::new();
    loop {
        render_text.clear();
        render_text.push_str(stacked_text.as_str());

        if step == 5 {
            exit_tui();
            install_service();
            break;
        }

        match step {
            0 => {
                render_text
                    .push_str("▶ config file already exists. Do you want to overwrite? (y/n): ");

                if overwrite {
                    render_text.push_str("y");
                } else {
                    render_text.push_str("n");
                }
            }
            1 => {
                render_text.push_str("▶ Enter Slack App Token: ");
                render_text.push_str(slack_app_token.as_str());
            }
            2 => {
                render_text.push_str("▶ Enter Slack Channel ID: ");
                render_text.push_str(&slack_channel_id);
            }
            3 => {
                render_text.push_str(format!("▶ Enter Disk Threshold: {disk_threshold}%").as_str());
            }
            4 => {
                render_text.push_str("▶ Do you want to install a daemon? (y/n): ");

                if install_daemon {
                    render_text.push_str("y");
                } else {
                    render_text.push_str("n");
                }
            }
            _ => {}
        }

        let block = Block::default()
            .title("Initialize Config")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(render_text.clone()).block(block);

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(paragraph, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match step {
                        0 => match key.code {
                            KeyCode::Char('y') => {
                                overwrite = true;
                            }
                            KeyCode::Char('n') => {
                                overwrite = false;
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Enter => {
                                if step == 0 {
                                    step = 1;
                                }
                            }
                            _ => {}
                        },
                        1 => match key.code {
                            KeyCode::Backspace => {
                                slack_app_token.pop();
                            }
                            KeyCode::Char(c) => {
                                slack_app_token.push(c);
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Enter => {
                                if step == 1 {
                                    step = 2;
                                    stacked_text.push_str(
                                        format!("Slack App Token: {slack_app_token}\n").as_str(),
                                    );
                                }
                            }
                            _ => {}
                        },
                        2 => match key.code {
                            KeyCode::Backspace => {
                                slack_channel_id.pop();
                            }
                            KeyCode::Char(c) => {
                                slack_channel_id.push(c);
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Enter => {
                                if step == 2 {
                                    step = 3;
                                    stacked_text.push_str(
                                        format!("Slack Channel ID: {slack_channel_id}\n").as_str(),
                                    );
                                }
                            }
                            _ => {}
                        },
                        3 => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Backspace => {
                                disk_threshold /= 10;
                            }
                            KeyCode::Char(c) => {
                                if let Some(digit) = c.to_digit(10) {
                                    disk_threshold = disk_threshold * 10 + (digit);
                                    if disk_threshold > 100 {
                                        disk_threshold %= 100;
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Enter => {
                                if step == 3 {
                                    let config = config::ConfigFile {
                                        slack_app_token: slack_app_token.clone(),
                                        slack_channel_id: slack_channel_id.clone(),
                                        disk_threshold: disk_threshold as u8,
                                    };
                                    stacked_text.push_str(
                                        format!("Disk Threshold: {disk_threshold}%\n").as_str(),
                                    );
                                    if let Err(error) = config.save() {
                                        exit_tui();
                                        panic!("Failed to save config file: {}", error);
                                    }
                                    stacked_text.push_str("Config file saved\n");

                                    step = 4;
                                }
                            }
                            _ => {}
                        },
                        4 => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('y') => {
                                install_daemon = true;
                            }
                            KeyCode::Char('n') => {
                                install_daemon = false;
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Enter => {
                                if step == 4 {
                                    step = 5;
                                    stacked_text.push_str("Install Daemon: ");
                                    if install_daemon {
                                        stacked_text.push_str("y\n");
                                    } else {
                                        stacked_text.push_str("n\n");
                                    }
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
