use std::collections::BTreeMap;
use std::ffi::CStr;
use std::fs;
use std::path::PathBuf;

use crate::api;

pub fn path() -> Option<PathBuf> {
    let ptr = unsafe { (api::hachimi_get_base_dir())() };
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().ok()?;
    Some(PathBuf::from(s).join("dictionary.json"))
}

fn load() -> BTreeMap<String, String> {
    let Some(path) = path() else {
        crate::logging::warn("dictionary::load: hachimi_get_base_dir returned null, using empty dictionary");
        return BTreeMap::new();
    };
    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(dict) => dict,
            Err(e) => {
                crate::logging::warn(&format!("dictionary::load: failed to parse {}: {e}, using empty dictionary", path.display()));
                BTreeMap::new()
            }
        },
        Err(_) => {
            crate::logging::info(&format!("dictionary::load: no dictionary file at {}, creating empty one", path.display()));
            let _ = fs::write(&path, "{}");
            BTreeMap::new()
        }
    }
}

pub fn build_glossary(text: &str) -> Option<String> {
    let mut glossary = String::from("[glossary]\n");
    let mut hit = false;
    for (jp, en) in load() {
        if text.contains(&jp) {
            glossary.push_str(&format!("- {jp} : {en}\n"));
            hit = true;
        }
    }
    hit.then_some(glossary)
}
