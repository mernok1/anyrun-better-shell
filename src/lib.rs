use std::{env, fs, process::Command};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    prefix: String,
    shell: Option<String>,
    stdout: String,
    max_line: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: ":bsh".to_string(),
            shell: None,
            stdout: "".to_string(),
            max_line: 65535,
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/shell.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Better Shell".into(),
        icon: "utilities-terminal".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &mut Config) -> RVec<Match> {
    if input.starts_with(&config.prefix) {
        let (_, command) = input.split_once(&config.prefix).unwrap();
        if !command.is_empty() {
            config.stdout = command.trim().to_string();

            let output = Command::new(
                config.shell.clone().unwrap_or_else(|| {
                    env::var("SHELL").unwrap_or_else(|_| {
                        "The shell could not be determined!".to_string()
                    })
                })
            )
            .arg("-c")
            .arg(format!("{} | head -n {}",command.trim().to_string(), config.max_line))
            .output()
            .expect("Command not running");

            vec![Match {
                title: config.stdout.clone().into(),
                description: ROption::RSome(
                   RString::from_utf8(output.stdout).unwrap(),
                ),
                use_pango: false,
                icon: ROption::RNone,
                id: ROption::RNone,
            }]
            .into()
        } else {
            RVec::new()
        }
    } else {
        RVec::new()
    }
}

#[handler]
fn handler(_selection: Match) -> HandleResult {
        HandleResult::Close
        // HandleResult::Close;

}