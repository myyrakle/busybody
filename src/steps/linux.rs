use std::{path::PathBuf, process::Command, str::FromStr};

const SERVICE_PATH: &str = "/etc/systemd/system/nosyman.service";

pub fn create_service_if_not_exists() {
    let path = PathBuf::from_str(SERVICE_PATH).unwrap();
    if !path.exists() {
        let output = Command::new("echo").arg("$USER").output().unwrap();
        let username = String::from_utf8(output.stdout).unwrap().trim().to_string();

        let service_content = format!(
            r#"[Unit]
Description=Nosyman

[Service]
Type=simple
Restart=on-failure
RestartSec=1
User={username}
WorkingDirectory=/home/{username}/.cargo/bin
ExecStart=/home/{username}/.cargo/bin/nosyman

[Install]
WantedBy=network-online.target"#
        );

        std::fs::write(SERVICE_PATH, service_content).unwrap();
    }
}
