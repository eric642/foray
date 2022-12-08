mod config;

use std::{fs, path::Path};

use crate::config::config::Config;

fn main() {
    let cfg_file = fs::read(Path::new("src/config/config.toml")).expect("read config file failed");
    let cfg: Config = toml::from_slice(&cfg_file).expect("parse config file failed");
    println!("{:?}", cfg);
}
