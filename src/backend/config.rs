use directories::ProjectDirs;
use serde_derive::Deserialize;
use std::fs;
use toml::value::Array;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) subscriptions: Array,
}

pub fn load_config() -> Option<Config> {
    let project = ProjectDirs::from("com", "j0lol", "rs-youtube").unwrap();
    let mut config_dir = project.config_dir();

    fs::create_dir_all(config_dir).ok()?;

    let config_path = format!("{}/config.toml", config_dir.to_str().unwrap());
    config_dir = std::path::Path::new(&config_path);

    Some(
        toml::from_str(&fs::read_to_string(config_dir).ok()?).unwrap_or(Config {
            subscriptions: Array::new(),
        }),
    )
}
