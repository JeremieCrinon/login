use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize)]
pub struct Config {
    pub app_name: String,
    pub app_display_name: String,
    pub api_url: String,
}

const CONFIG_STR: &str = include_str!("../config.toml");

lazy_static! {
    pub static ref CONFIG: Config = toml::from_str(CONFIG_STR).unwrap();
}

