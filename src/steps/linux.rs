use std::process::Command;

const SERVICE_PATH: &str = "/etc/systemd/system/nosyman.service";

pub fn install_service() {
    let service_content = format!(
        r#"[Unit]
Description=Nosyman

[Service]
Type=simple
Restart=on-failure
RestartSec=1
User=root
WorkingDirectory=/usr/bin
ExecStart=/usr/bin/nosyman

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

    let output = Command::new("systemctl")
        .arg("restart")
        .arg("nosyman.service")
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
