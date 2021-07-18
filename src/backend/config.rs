use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml::value::Array;
use toml::Value;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub(crate) youtube: YouTube,
    pub(crate) options: Options,
    pub(crate) twitch: Twitch,
}

#[derive(Deserialize, Serialize)]
pub struct YouTube {
    pub(crate) subscriptions: Array,
}

#[derive(Deserialize, Serialize)]
pub struct Twitch {
    pub(crate) follows: Array,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub(crate) video_player: Value,
}


pub fn load_config() -> Option<Config> {
    let project: ProjectDirs = ProjectDirs::from("com", "j0lol", "rs-youtube").unwrap();
    let config_dir: &Path = project.config_dir();

    fs::create_dir_all(config_dir).ok()?;

    let config_path = format!("{}/config.toml", config_dir.to_str().unwrap());
    let config_path = std::path::Path::new(&config_path);

    let output = match fs::read_to_string(config_path) {
        Ok(value) => value,
        Err(_) => initial_config(),
    };
    let output = output.as_str();

    match toml::from_str(output) {
        Ok(value) => Some(value),
        Err(err) => {
            println!("\n\n\nYour config file, at {}, is not valid.\nIf this happened after a program update, a new field may have been added.\nPlease see the default config at: https://github.com/j0lol/rs-youtube/blob/main/config.toml.default and make changes accordingly.\nSorry for breaking your config!\n\n\n", config_path.to_str().unwrap());
            println!("Heres your error!");

            panic!("Could not parse config file! Error: {}", err);        }
    }
}

pub fn write_config(config: Config) {
    let project: ProjectDirs = ProjectDirs::from("com", "j0lol", "rs-youtube").unwrap();
    let config_dir: &Path = project.config_dir();
    fs::create_dir_all(config_dir).unwrap();

    let config_path = format!("{}/config.toml", config_dir.to_str().unwrap());
    let config_path = std::path::Path::new(&config_path);

    let string = toml::to_string(&config).unwrap();
    fs::write(config_path, string.as_bytes()).unwrap();
}

pub fn initial_config() -> String {
    let project: ProjectDirs = ProjectDirs::from("com", "j0lol", "rs-youtube").unwrap();
    let config_dir: &Path = project.config_dir();
    fs::create_dir_all(config_dir).unwrap();

    let config_path = format!("{}/config.toml", config_dir.to_str().unwrap());
    let config_path = std::path::Path::new(&config_path);

    let string = toml::to_string(&Config {
        youtube: YouTube { subscriptions: Array::new() },
        options: Options {video_player: toml::Value::String("mpv -ao=alsa -v --ytdl-format='[height<420]'".to_string())
    },
        twitch: Twitch { follows: Array::new() },
    })
        .unwrap();
    fs::write(config_path, string.as_bytes()).unwrap();
    load_config();
    string
}

pub fn is_subscribed(channel_id: String) -> bool {
    let config = load_config().unwrap();
    config.youtube
        .subscriptions
        .contains(&toml::Value::from(channel_id))
}

pub fn subscribe(channel_id: String) {
    let mut config = load_config().unwrap();
    config.youtube.subscriptions.push(toml::Value::from(channel_id));
    write_config(config);
}

pub fn unsubscribe(channel_id: String) {
    let mut config = load_config().unwrap();
    let remove_value = toml::Value::from(channel_id);
    config.youtube.subscriptions.retain(|x| *x != remove_value);
    write_config(config);
}

pub fn is_following(channel_id: String) -> bool {
    let config = load_config().unwrap();
    config.twitch.follows.contains(&toml::Value::from(channel_id))
}

pub fn follow(channel_id: String) {
    let mut config = load_config().unwrap();
    config.twitch.follows.push(toml::Value::from(channel_id));
    write_config(config);
}

pub fn unfollow(channel_id: String) {
    let mut config = load_config().unwrap();
    let remove_value = toml::Value::from(channel_id);
    config.twitch.follows.retain(|x| *x != remove_value);
    write_config(config);
}
