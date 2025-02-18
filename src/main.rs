use std::process::Command;

use confy;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
struct Config {
    temperature: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config { temperature: 6500 }
    }
}

fn is_sct_installed() -> bool {
    Command::new("which")
        .arg("sct")
        .output()
        .expect("Failed to execute which")
        .status
        .success()
}


fn main() {

    let mut args = std::env::args();
    let _program = args.next().expect("No program name");

    if !is_sct_installed() {
        eprintln!("sct is not installed");
        std::process::exit(1);
    }

    let config: Config = confy::load("sctw", None).unwrap_or_else(|_| 
        Config::default()
    );

    let temperature = args.next().map(|arg| arg.parse::<u32>().expect("Invalid temperature"))
        .unwrap_or(config.temperature);

    let temperature = temperature.clamp(1_000, 10_000);

    let config = Config { temperature };

    Command::new("sct")
        .arg(temperature.to_string())
        .spawn()
        .expect("Failed to execute sct");

    confy::store("sctw", None, config).expect("Failed to store config");

}

