use std::hint::spin_loop;

use crate::config::load_config;

pub fn send_to_slack(disk_usage: u8, disk_threshold: u8) {
    let config_file = load_config();
    let slack_app_token = config_file.slack_app_token;
    let slack_channel_id = config_file.slack_channel_id;

    let message = format!(
        r#"<!here> 디스크 사용량이 임계치를 초과했습니다. 정리가 필요합니다. 
({disk_usage}% > {disk_threshold}%)"#,
    );

    let client = reqwest::blocking::Client::new();

    let request_body =
        serde_json::json!({ "text": message, "channel":slack_channel_id  }).to_string();

    let res = client
        .post("https://slack.com/api/chat.postMessage")
        .header("content-type", "application/json")
        .bearer_auth(slack_app_token)
        .body(request_body)
        .send()
        .unwrap();

    println!("{:?}", res.text().unwrap());
}

pub fn run() {
    let config_file = load_config();
    let mut latest_disk_usage = 0;
    let disk_threshold = config_file.disk_threshold;

    let mut danger = false;
    loop {
        let output = std::process::Command::new("df")
            .arg("-h")
            .arg("/")
            .output()
            .unwrap();

        let output = String::from_utf8(output.stdout).unwrap();

        let rows = output
            .split("\n")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let disk_usage: u8 = rows[1].split_whitespace().collect::<Vec<&str>>()[4]
            .replace("%", "")
            .parse()
            .unwrap();

        // 사용량이 같으면 무시
        if disk_usage == latest_disk_usage {
            spin_loop();
            continue;
        }

        latest_disk_usage = disk_usage;

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let stable = if disk_usage < disk_threshold {
            if danger {
                danger = false;
            }
            "stable"
        } else {
            if !danger {
                danger = true;
                send_to_slack(disk_usage, disk_threshold);
            }
            "unstable"
        };

        println!("[{now}] {stable} ({disk_usage}% < {disk_threshold}%)");

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
