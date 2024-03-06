use std::{fs, process::Command};

const SERVICE_PATH: &str = "/etc/systemd/system/nosyman.service";
const SHELL_PATH: &str = "/usr/bin/nosyman.sh";

pub fn install_service() {
    let username = std::env::var("USER").unwrap();

    fs::write(
        SHELL_PATH,
        format!(
            r#"#!/bin/bash
/home/{username}/.cargo/bin/nosyman start
        "#
        ),
    )
    .unwrap();

    let service_content = format!(
        r#"[Unit]
Description=Nosyman

[Service]
Type=simple
Restart=on-failure
RestartSec=1
User={username}
WorkingDirectory=/home/{username}/.cargo/bin
ExecStart=/usr/bin/nosyman.sh

[Install]
WantedBy=network-online.target"#
    );

    std::fs::write(SERVICE_PATH, service_content).unwrap();

    let output = Command::new("systemctl")
        .arg("enable")
        .arg("--now")
        .arg("nosyman.service")
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
