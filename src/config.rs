use anyhow::{anyhow, Result};
use std::{fs, path};

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserchromeConfig {
    pub key: String,
    pub value: String,
    pub raw: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Userchrome {
    pub name: String,
    #[serde(default)]
    pub source: String,

    /// deprecated
    pub clone_url: Option<String>,

    #[serde(default)]
    pub configs: Vec<UserchromeConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub profile: String,

    #[serde(default)]
    pub userchromes: Vec<Userchrome>,
}

fn get_config_path() -> Result<path::PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        Ok(config_dir.join("nyoom.toml"))
    } else {
        Err(anyhow!("unable to locate config dirs"))
    }
}

pub fn get_config() -> Result<Config> {
    let path = get_config_path()?;
    let f = match path.exists() {
        true => fs::read_to_string(path)?,
        false => "".into(),
    };
    let mut config: Config = toml::from_str(&f)?;

    for uc in &mut config.userchromes {
        if let Some(old_clone_url) = &uc.clone_url {
            uc.source = old_clone_url
                .replace("https://github.com/", "github:")
                .replace(".git", "");
            uc.clone_url = None
        }
    }

    set_config(&config)?;

    Ok(config)
}

pub fn set_config(config: &Config) -> Result<()> {
    let serialized = toml::to_string_pretty(&config)?;
    fs::write(get_config_path()?, serialized)?;

    Ok(())
}

pub fn print_userchrome(userchrome: &Userchrome, short: bool) {
    println!(
        "{} {} {}",
        "·".cyan(),
        userchrome.name.cyan(),
        userchrome.source.dimmed()
    );

    let slice_len = match short {
        true => userchrome.configs.len().min(3),
        false => userchrome.configs.len(),
    };

    for c in &userchrome.configs[..slice_len] {
        println!(
            "   {}: {}{}",
            c.key.magenta(),
            c.value,
            match c.raw {
                true => " (raw)".dimmed(),
                false => "".into(),
            }
        );
    }

    if short && userchrome.configs.len() > 3 {
        println!(
            "{}",
            format!("   and {} more", userchrome.configs.len() - 3).dimmed()
        );
    }
}