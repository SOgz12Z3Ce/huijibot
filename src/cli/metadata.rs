use std::env;
use std::fs;
use std::path::PathBuf;

use clap::Args;
use serde::Deserialize;
use serde::Serialize;

pub(crate) fn root_path() -> PathBuf {
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

fn hujibot_path() -> PathBuf {
    root_path().join(".huijibot")
}

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub(crate) struct HuijibotConfig {
    #[arg(short, long)]
    pub(crate) site: Option<String>,

    #[arg(short, long)]
    pub(crate) username: Option<String>,

    #[arg(short, long)]
    pub(crate) password: Option<String>,

    #[arg(short, long)]
    pub(crate) auth_key: Option<String>,
}

pub(crate) fn config_path() -> PathBuf {
    hujibot_path().join("config")
}

pub(crate) fn config() -> HuijibotConfig {
    let config_path = config_path();
    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        toml::from_str("").unwrap()
    }
}
