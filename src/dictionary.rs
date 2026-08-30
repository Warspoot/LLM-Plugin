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

// Longest-match-wins over non-overlapping spans, not plain substring containment - e.g. "カレン"
// is itself a substring of "カレンブーケドール", and including both would hand the model two
// conflicting hints for the same word, which it then mashes together with no space.
pub fn build_glossary(text: &str) -> Option<String> {
    let dict = load();

    let mut occurrences: Vec<(usize, usize, &str, &str)> = Vec::new();
    for (jp, en) in &dict {
        for (start, matched) in text.match_indices(jp.as_str()) {
            occurrences.push((start, start + matched.len(), jp.as_str(), en.as_str()));
        }
    }
    occurrences.sort_by_key(|&(start, end, ..)| std::cmp::Reverse(end - start));

    let mut accepted_spans: Vec<(usize, usize)> = Vec::new();
    let mut hits: Vec<(&str, &str)> = Vec::new();
    for (start, end, jp, en) in occurrences {
        if accepted_spans.iter().any(|&(s, e)| start < e && s < end) {
            continue;
        }
        accepted_spans.push((start, end));
        hits.push((jp, en));
    }

    if hits.is_empty() {
        return None;
    }
    hits.sort();

    let mut glossary = String::from(
        "[glossary]\n\
        Use the English term exactly as written below, character-for-character, for every \
        listed phrase - do not alter spacing, punctuation, or capitalization:\n"
    );
    for (jp, en) in hits {
        glossary.push_str(&format!("- {jp} : {en}\n"));
    }
    Some(glossary)
}

pub fn is_known(text: &str) -> bool {
    load().contains_key(text)
}
