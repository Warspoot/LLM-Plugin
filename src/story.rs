use std::ffi::CString;
use std::ffi::c_void;
use std::sync::OnceLock;

use crate::api;
use crate::api::Il2CppClass;
use crate::api::FieldInfo;
use crate::cache;
use crate::il2cpp::{list_len, list_ref_at, read_il2cpp_string};
use crate::llm;

pub fn install() {
    let class = story_timeline_data_class();
    if class.is_null() {
        crate::logging::warn("story::install: StoryTimelineData class not found");
    }
    else {
        crate::logging::info("story::install: StoryTimelineData class found");
    }

    let title = title_field();
    if title.is_null() {
        crate::logging::warn("story::install: StoryTimeLineData.Title field not found");
    }
    else {
        crate::logging::info("story::install: StoryTimeLineData.Title field found");
    }

    let block_list = block_field_list();
    if block_list.is_null() {
        crate::logging::warn("story::install: StoryTimeLineData.BlockList field not found");
    }
    else {
        crate::logging::info("story::install: StoryTimeLineData.BlockList field found");
    }

    let class = story_timeline_block_data_class();
    if class.is_null() {
        crate::logging::warn("story::install: StoryTimelineBlockData class not found");
    }
    else {
        crate::logging::info("story::install: StoryTimelineBlockData class found");
        let field = text_track_field();
        if field.is_null() {
            crate::logging::warn("story::install: StoryTimelineBlockData.TextTrack field not found");
        }
        else {
            crate::logging::info("story::install: StoryTimelineBlockData.TextTrack field found");
        }
    }

    let class = story_timeline_track_data_class();
    if class.is_null() {
        crate::logging::warn("story::install: StoryTimelineTrackData class not found");
    }
    else {
        crate::logging::info("story::install: StoryTimelineTrackData class found");
        let field = clip_list_field();
        if field.is_null() {
            crate::logging::warn("story::install: StoryTimelineTrackData.ClipList field not found");
        }
        else {
            crate::logging::info("story::install: StoryTimelineTrackData.ClipList field found");
        }
    }

    let class = story_timeline_text_clip_data_class();
    if class.is_null() {
        crate::logging::warn("story::install: StoryTimelineTextClipData class not found");
    }
    else {
        crate::logging::info("story::install: StoryTimelineTextClipData class found");
        let field = name_field();
        if field.is_null() {
            crate::logging::warn("story::install: StoryTimelineTextClipData.Name field not found");
        }
        else {
            crate::logging::info("story::install: StoryTimelineTextClipData.Name field found");
        }
        let field = text_field();
        if field.is_null() {
            crate::logging::warn("story::install: StoryTimelineTextClipData.Text field not found");
        }
        else {
            crate::logging::info("story::install: StoryTimelineTextClipData.Text field found");
        }
    }
}

pub fn story_timeline_data_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineData") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn title_field() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Title") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn block_field_list() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("BlockList") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn story_timeline_block_data_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineBlockData") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn text_track_field() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_block_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("TextTrack") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn story_timeline_track_data_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineTrackData") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn clip_list_field() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_track_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("ClipList") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn story_timeline_text_clip_data_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineTextClipData") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn name_field() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_text_clip_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Name") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn text_field() -> *mut FieldInfo  {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init( || {
        let class = story_timeline_text_clip_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Text") else {return 0};
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize}
    });
    addr as *mut FieldInfo
}

pub fn choice_data_list_field() -> *mut FieldInfo {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init(|| {
        let class = story_timeline_text_clip_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("ChoiceDataList") else { return 0 };
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize }
    });
    addr as *mut FieldInfo
}

pub fn choice_data_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let parent = story_timeline_text_clip_data_class();
        if parent.is_null() {
            return 0
        }
        let Ok(name) = CString::new("ChoiceData") else { return 0 };
        unsafe { (api::il2cpp_find_nested_class())(parent, name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn choice_text_field() -> *mut FieldInfo {
    static FIELD: OnceLock<usize> = OnceLock::new();
    let addr = *FIELD.get_or_init(|| {
        let class = choice_data_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Text") else { return 0 };
        unsafe { (api::il2cpp_get_field_from_name())(class, name.as_ptr()) as usize }
    });
    addr as *mut FieldInfo
}

pub fn read_title(timeline_data: *mut api::Il2CppObject) -> String {
    let field = title_field();
    if field.is_null() || timeline_data.is_null() {
        return String::new();
    }

    let mut title_ptr: *mut api::Il2CppString = std::ptr::null_mut();
    unsafe {
        (api::il2cpp_get_field_value())(
            timeline_data,
            field,
            &mut title_ptr as *mut _ as *mut c_void,
        );
    }

    unsafe { read_il2cpp_string(title_ptr) }
}

pub fn unity_object_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("UnityEngine.CoreModule.dll") else {return 0};
        let Ok(namespace) = CString::new("UnityEngine") else {return 0};
        let Ok(name) = CString::new("Object") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn object_get_name_addr() -> *mut c_void {
    static METHOD: OnceLock<usize> = OnceLock::new();
    let addr = *METHOD.get_or_init(|| {
        let class = unity_object_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("get_name") else {return 0};
        unsafe { (api::il2cpp_get_method_addr())(class, name.as_ptr(), 0) as usize}
    });
    addr as *mut c_void
}

type ObjectGetNameFn = unsafe extern "C" fn(this: *mut api::Il2CppObject) -> *mut api::Il2CppString;

/// The story object's own unity `.name` (e.g. "storytimeline_02001011") used for the cache
pub fn read_object_name(obj: *mut api::Il2CppObject) -> String {
    let addr = object_get_name_addr();
    if addr.is_null() {
        return String::new();
    }
    unsafe {
        let f: ObjectGetNameFn = std::mem::transmute(addr);
        read_il2cpp_string(f(obj))
    }
}

// translation qol
pub fn needs_translation(text: &str) -> bool {
    text.chars().any(|c| {
        let code = c as u32;
        (0x3040..=0x309F).contains(&code)   // Hiragana
            || (0x30A0..=0x30FF).contains(&code) // Katakana
            || (0x4E00..=0x9FFF).contains(&code) // Kanji
    })
}

pub fn wrap_text(text: &str, line_width: usize) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if !current_line.is_empty() && current_line.chars().count() + 1 + word.chars().count() > line_width {
            lines.push(std::mem::take(&mut current_line));
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines.join(" \n")
}

fn write_translated_text(obj: *mut api::Il2CppObject, field: *mut FieldInfo, text: &str) {
    let Ok(c_text) = CString::new(text) else { return };
    let new_str = unsafe { (api::il2cpp_string_new())(c_text.as_ptr()) };
    if new_str.is_null() {
        return;
    }
    unsafe {
        (api::il2cpp_set_field_value())(obj, field, new_str as *const c_void);
    }
}

// read title and text, translates it, and writes each result back before moving on to the next
// block - fully synchronous/blocking. This freezes the render thread for the duration, but
// avoids the entire class of races that came with translating on a background thread (a story
// being abandoned or superseded mid-translation, writing into a block that's already on screen,
// etc.) since nothing else can run while we're blocked.
pub fn process(timeline_data: *mut api::Il2CppObject) {
    if !crate::config::get().enabled {
        return;
    }

    let title = read_title(timeline_data);
    let story_name = read_object_name(timeline_data);
    crate::logging::info(&format!("story::process: Title = {title:?}, name = {story_name:?}"));

    let mut block_list_ptr: *mut c_void = std::ptr::null_mut();
    unsafe {
        (api::il2cpp_get_field_value())(
            timeline_data,
            block_field_list(),
            &mut block_list_ptr as *mut _ as *mut c_void,
        );
    }
    let count = unsafe { list_len(block_list_ptr) };
    crate::logging::info(&format!("story::process: BlockList has {count} blocks"));

    let mut block_dicts: Vec<cache::BlockDict> = Vec::new();

    for i in 0..count {
        let block = unsafe { list_ref_at(block_list_ptr, i) };
        if block.is_null() {
            block_dicts.push(cache::BlockDict::default());
            continue;
        }

        let mut text_track: *mut c_void = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(block as *mut api::Il2CppObject, text_track_field(), &mut text_track as *mut _ as *mut c_void);
        }
        if text_track.is_null() {
            block_dicts.push(cache::BlockDict::default());
            continue; // block has no dialogue
        }

        let mut clip_list_ptr: *mut c_void = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(text_track as *mut api::Il2CppObject, clip_list_field(), &mut clip_list_ptr as *mut _ as *mut c_void);
        }

        let clip = unsafe { list_ref_at(clip_list_ptr, 0) };
        if clip.is_null() {
            block_dicts.push(cache::BlockDict::default());
            continue;
        }

        // "モノローグ" ("Monologue") and "<username>" are sentinels, not real speaker names
        let mut name_ptr: *mut api::Il2CppString = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(clip as *mut api::Il2CppObject, name_field(), &mut name_ptr as *mut _ as *mut c_void);
        }
        let name = unsafe { read_il2cpp_string(name_ptr) };
        let name_dict_value: Option<String> = if name == "モノローグ" {
            crate::logging::info(&format!("story::process: block {i} name = {name:?} -> blanking monologue label"));
            write_translated_text(clip as *mut api::Il2CppObject, name_field(), "");
            Some(String::new())
        } else if name == "<username>" || name.is_empty() {
            None
        } else if needs_translation(&name) || crate::dictionary::is_known(&name) {
            if let Some(translated) = llm::translate_name(&name) {
                crate::logging::info(&format!("story::process: block {i} name translated = {translated:?}"));
                write_translated_text(clip as *mut api::Il2CppObject, name_field(), &translated);
                Some(translated)
            } else {
                None
            }
        } else {
            Some(name)
        };

        let mut text_ptr: *mut api::Il2CppString = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(clip as *mut api::Il2CppObject, text_field(), &mut text_ptr as *mut _ as *mut c_void);
        }

        let text = unsafe { read_il2cpp_string(text_ptr) };
        let needs_tl = needs_translation(&text) || crate::dictionary::is_known(&text);
        crate::logging::info(&format!("story::process: block {i} text = {text:?} (needs_translation={needs_tl})"));

        let mut choice_list_ptr: *mut c_void = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(clip as *mut api::Il2CppObject, choice_data_list_field(), &mut choice_list_ptr as *mut _ as *mut c_void);
        }
        let choice_count = unsafe { list_len(choice_list_ptr) };
        let mut choice_texts: Vec<String> = Vec::new();
        for j in 0..choice_count {
            let choice_obj = unsafe { list_ref_at(choice_list_ptr, j) };
            if choice_obj.is_null() {
                choice_texts.push(String::new());
                continue;
            }

            let mut choice_text_ptr: *mut api::Il2CppString = std::ptr::null_mut();
            unsafe {
                (api::il2cpp_get_field_value())(choice_obj as *mut api::Il2CppObject, choice_text_field(), &mut choice_text_ptr as *mut _ as *mut c_void);
            }
            let choice_text = unsafe { read_il2cpp_string(choice_text_ptr) };
            let choice_needs_tl = needs_translation(&choice_text) || crate::dictionary::is_known(&choice_text);
            crate::logging::info(&format!("story::process: block {i} choice {j} text = {choice_text:?} (needs_translation={choice_needs_tl})"));

            if choice_needs_tl {
                if let Some(translated) = llm::translate(&choice_text) {
                    let wrapped = wrap_text(&translated, 45);
                    crate::logging::info(&format!("story::process: block {i} choice {j} translated = {wrapped:?}"));
                    write_translated_text(choice_obj as *mut api::Il2CppObject, choice_text_field(), &wrapped);
                    choice_texts.push(wrapped);
                    continue;
                }
            }
            choice_texts.push(choice_text);
        }

        if needs_tl {
            if let Some(translated) = llm::translate(&text) {
                let wrapped = wrap_text(&translated, 45);
                crate::logging::info(&format!("story::process: block {i} translated = {wrapped:?}"));
                write_translated_text(clip as *mut api::Il2CppObject, text_field(), &wrapped);
                block_dicts.push(cache::BlockDict { name: name_dict_value, text: Some(wrapped), choice_data_list: choice_texts, ..Default::default() });
            } else {
                block_dicts.push(cache::BlockDict { name: name_dict_value, choice_data_list: choice_texts, ..Default::default() });
            }
        } else {
            block_dicts.push(cache::BlockDict { name: name_dict_value, text: Some(text), choice_data_list: choice_texts, ..Default::default() });
        }
    }

    let dict = cache::Dict {
        title: if title.is_empty() { None } else { Some(title) },
        text_block_list: block_dicts,
        no_wrap: false,
    };
    cache::save_dict(&story_name, &dict);
}