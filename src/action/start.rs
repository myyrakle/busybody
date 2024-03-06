use crate::config::load_config;

pub fn run() {
    let config_file = load_config();
    let disk_threshold = config_file.disk_threshold;

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

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let stable = if disk_usage < disk_threshold {
            "stable"
        } else {
            "unstable"
        };

        println!("[{now}] {stable} ({disk_usage}% < {disk_threshold}%)");

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
