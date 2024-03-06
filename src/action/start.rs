use crate::config::load_config;

pub fn run() {
    let _config_file = load_config();

    loop {
        println!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
