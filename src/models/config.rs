use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_ui")]
    pub ui: UiConfig,
    #[serde(default = "default_calendar")]
    pub calendar: CalendarConfig,
}

#[derive(Serialize, Deserialize)]
pub struct UiConfig {
    pub default_view: String,
    pub week_start_day: String,
    pub time_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarConfig {
    pub visible_hours_start: String,
    pub visible_hours_end: String,
    pub slot_interval_minutes: u32,
}

fn default_ui() -> UiConfig {
    UiConfig {
        default_view: "month".to_string(),
        week_start_day: "monday".to_string(),
        time_format: "24h".to_string(),
    }
}

fn default_calendar() -> CalendarConfig {
    CalendarConfig {
        visible_hours_start: "06:00".to_string(),
        visible_hours_end: "18:00".to_string(),
        slot_interval_minutes: 30,
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: default_ui(),
            calendar: default_calendar(),
        }
    }
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = directories::ProjectDirs::from("com", "calendar-app", "calendar-app")
        .context("Failed to get config directory")?
        .config_dir()
        .to_path_buf();
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

