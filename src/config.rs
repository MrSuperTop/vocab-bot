use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let error_message =
            format!("Could not read the config file which sould be located at {path}");
        let data = fs::read_to_string(path).expect(&error_message);

        serde_json::from_str(&data).unwrap()
    }
}
