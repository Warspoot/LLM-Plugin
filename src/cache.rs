use std::ffi::CStr;
use std::fs;
use std::path::PathBuf;

use serde::Serialize;

use crate::api;

#[derive(Serialize, Default, Clone)]
pub struct BlockDict {
    pub name: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub choice_data_list: Vec<String>,
    #[serde(default)]
    pub color_text_info_list: Vec<String>,
}

#[derive(Serialize, Default, Clone)]
pub struct Dict {
    pub title: Option<String>,
    pub text_block_list: Vec<BlockDict>,
    pub no_wrap: bool,
}

fn base_dir() -> Option<PathBuf> {
    let ptr = unsafe { (api::hachimi_get_base_dir())() };
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().ok()?;
    Some(PathBuf::from(s))
}

fn cache_path(story_name: &str) -> Option<PathBuf> {
    let dir = base_dir()?.join("localized_data").join("assets").join("llm_cache");
    Some(dir.join(format!("{story_name}.json")))
}

pub fn save_dict(story_name: &str, dict: &Dict) {
    let Some(path) = cache_path(story_name) else {
        crate::logging::warn("cache::save_dict: hachimi_get_base_dir returned null, skipping save");
        return;
    };

    let Some(dir) = path.parent() else { return };
    if let Err(e) = fs::create_dir_all(dir) {
        crate::logging::warn(&format!("cache::save_dict: failed to create directory {}: {e}", dir.display()));
        return;
    }

    let json = match serde_json::to_string_pretty(dict) {
        Ok(j) => j,
        Err(e) => {
            crate::logging::warn(&format!("cache::save_dict: failed to serialize dict: {e}"));
            return;
        }
    };

    match fs::write(&path, json) {
        Ok(()) => crate::logging::info(&format!("cache::save_dict: saved {}", path.display())),
        Err(e) => crate::logging::warn(&format!("cache::save_dict: failed to write {}: {e}", path.display())),
    }
}
