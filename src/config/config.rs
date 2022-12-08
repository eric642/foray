use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Listening {
    pub ip: Vec<String>,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub listening: Listening,
}
