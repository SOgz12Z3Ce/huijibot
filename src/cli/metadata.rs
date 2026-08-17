use std::env;
use std::fs;
use std::path::PathBuf;

use clap::Args;
use serde::Deserialize;
use serde::Serialize;

pub(super) fn root_path() -> PathBuf {
    let mut cur_path = fs::canonicalize(env::current_dir().unwrap()).unwrap();
    loop {
        let huijibot_path = cur_path.join(".huijibot");
        if huijibot_path.exists() {
            return cur_path;
        }
        match cur_path.parent() {
            Some(path) => cur_path = path.to_path_buf(),
            None => panic!("Not initialized!"),
        }
    }
}

pub(super) fn hujibot_path() -> PathBuf {
    root_path().join(".huijibot")
}

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub(super) struct HuijibotConfig {
    #[arg(short, long)]
    pub(super) site: Option<String>,

    #[arg(short, long)]
    pub(super) username: Option<String>,

    #[arg(short, long)]
    pub(super) password: Option<String>,
}

pub(super) fn config_path() -> PathBuf {
    hujibot_path().join("config")
}

pub(super) fn config() -> HuijibotConfig {
    let config_path = config_path();
    let content = fs::read_to_string(config_path).unwrap();
    toml::from_str(&content).unwrap()
}
