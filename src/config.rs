use std::ffi::CStr;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use crate::api;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub enabled: bool,
    pub endpoint: String,
    pub model: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub top_k: i32,
    pub min_p: f32,
    pub repetition_penalty: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://127.0.0.1:1234/v1/chat/completions".to_owned(),
            model: "local-model".to_owned(),
            system_prompt: "Translate the following Japanese video game dialogue into natural \
                English. Reply with only the translation, no notes or explanation.".to_owned(),
            temperature: 0.3,
            top_k: 40,
            min_p: 0.05,
            repetition_penalty: 1.1,
        }
    }
}

pub fn path() -> Option<PathBuf> {
    let ptr = unsafe { (api::hachimi_get_base_dir())() };
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().ok()?;
    Some(PathBuf::from(s).join("llm_plugin_config.json"))
}

fn load() -> Config {
    let Some(path) = path() else {
        crate::logging::warn("config::load: hachimi_get_base_dir returned null, using defaults");
        return Config::default();
    };
    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(config) => {
                crate::logging::info(&format!("config::load: loaded {}", path.display()));
                config
            }
            Err(e) => {
                crate::logging::warn(&format!("config::load: failed to parse {}: {e}, using defaults", path.display()));
                Config::default()
            }
        },
        Err(_) => {
            crate::logging::info(&format!("config::load: no config file at {}, using defaults", path.display()));
            Config::default()
        }
    }
}

fn save(config: &Config) {
    let Some(path) = path() else {
        crate::logging::warn("config::save: hachimi_get_base_dir returned null, skipping save");
        return;
    };
    let json = match serde_json::to_string_pretty(config) {
        Ok(j) => j,
        Err(e) => {
            crate::logging::warn(&format!("config::save: failed to serialize: {e}"));
            return;
        }
    };
    match fs::write(&path, json) {
        Ok(()) => crate::logging::info(&format!("config::save: saved {}", path.display())),
        Err(e) => crate::logging::warn(&format!("config::save: failed to write {}: {e}", path.display())),
    }
}

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

fn cell() -> &'static Mutex<Config> {
    CONFIG.get_or_init(|| Mutex::new(load()))
}

pub fn get() -> Config {
    cell().lock().unwrap().clone()
}

pub fn set(new: Config) {
    save(&new);
    *cell().lock().unwrap() = new;
}

pub fn reload() -> Config {
    let reloaded = load();
    *cell().lock().unwrap() = reloaded.clone();
    reloaded
}
