use std::{fs, path::PathBuf, sync::Arc};

use colored::Colorize;
use rprompt::prompt_reply;
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

pub struct Environment;

impl TypeMapKey for Environment {
    type Value = Env;
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ConfigFile {
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Env {
    token: String,
    lavalink: Arc<RwLock<LavalinkEnv>>,
}

#[derive(Debug, Clone)]
pub struct LavalinkEnv {
    host: String,
    port: u16,
    password: String,
}

impl Env {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub async fn lavalink(&self) -> LavalinkEnv {
        let lavalink = self.lavalink.read().await;
        lavalink.clone()
    }

    pub async fn update_lavalink(&self, host: String, port: u16, password: String) {
        let mut lavalink = self.lavalink.write().await;
        lavalink.host = host;
        lavalink.port = port;
        lavalink.password = password;
    }

    fn setup_token() -> String {
        loop {
            let path = Self::get_config_path().unwrap_or(PathBuf::from("config.toml"));
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if let Ok(config) = toml::from_str::<ConfigFile>(&content) {
                        if let Some(token) = config.token {
                            return token;
                        }
                    }
                    fs::remove_file(&path).expect("Failed to remove config file");
                }
                Err(_) => {
                    let prompt = format!(
                        "Please enter your {}: ",
                        "Bot Token".truecolor(69, 79, 191).bold()
                    );
                    let mut token_str = String::new();
                    while let Ok(token) = prompt_reply(&prompt) {
                        if token.is_empty() {
                            println!("token cannot be empty. Please try again.");
                            continue;
                        }
                        if serenity::all::token::validate(&token).is_err() {
                            println!("token is not a valid token. Please try again.");
                            continue;
                        }
                        token_str = token;
                    }

                    let config = ConfigFile {
                        token: Some(token_str.clone()),
                    };
                    let Ok(config) = toml::to_string(&config) else {
                        println!("Failed to serialize config. Please try again.");
                        continue;
                    };
                    fs::write(&path, config).expect("Failed to write config file");
                }
            }
        }
    }

    fn get_config_path() -> Option<PathBuf> {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let mut files = fs::read_dir(current_dir).expect("Failed to read directory");
        files.find_map(|entry| {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_file() && path.file_name().is_some_and(|name| name == "config.toml") {
                return Some(path);
            }
            None
        })
    }
}

impl Default for Env {
    fn default() -> Self {
        let token = Self::setup_token();
        let (host, port, password) = {
            (
                env!("LAVALINK_HOST").to_string(),
                env!("LAVALINK_PORT").parse().unwrap_or_default(),
                env!("LAVALINK_PASSWORD").to_string(),
            )
        };
        Self {
            token,
            lavalink: Arc::new(LavalinkEnv::new(host, port, password).into()),
        }
    }
}

impl LavalinkEnv {
    pub fn new(host: String, port: u16, password: String) -> Self {
        Self {
            host,
            port,
            password,
        }
    }

    pub fn hostname(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
