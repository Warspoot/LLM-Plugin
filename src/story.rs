use std::ffi::CString;
use std::ffi::c_void;
use std::sync::OnceLock;
use std::sync::Mutex;
use std::thread;

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

struct PendingWrite {
    obj: usize,
    field: usize,
    text: String,
}

static PENDING_WRITES: Mutex<Vec<PendingWrite>> = Mutex::new(Vec::new());
static MAIN_THREAD: OnceLock<usize> = OnceLock::new();

fn main_thread() -> *mut api::Il2CppThread {
    let addr = *MAIN_THREAD.get_or_init(|| unsafe { (api::il2cpp_get_main_thread())() as usize });
    addr as *mut api::Il2CppThread
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

unsafe extern "C" fn apply_pending_writes() {
    let mut pending = PENDING_WRITES.lock().unwrap();
    for write in pending.drain(..) {
        write_translated_text(write.obj as *mut api::Il2CppObject, write.field as *mut FieldInfo, &write.text);
    }
}

// translation cache logic
struct CurrentDict {
    story_name: String,
    dict: cache::Dict,
}

static CURRENT_DICT: Mutex<Option<CurrentDict>> = Mutex::new(None);

// choice option (by its index within that block's ChoiceDataList).
enum TranslationTarget {
    Text,
    Choice(usize),
}

fn update_and_save_dict(index: usize, target: &TranslationTarget, translated: &str) {
    let mut guard = CURRENT_DICT.lock().unwrap();
    let Some(current) = guard.as_mut() else { return };
    if let Some(block) = current.dict.text_block_list.get_mut(index) {
        match target {
            TranslationTarget::Text => block.text = Some(translated.to_owned()),
            TranslationTarget::Choice(choice_index) => {
                if let Some(slot) = block.choice_data_list.get_mut(*choice_index) {
                    *slot = translated.to_owned();
                }
            }
        }
    }
    cache::save_dict(&current.story_name, &current.dict);
}

struct PendingBlock {
    index: usize,
    obj: usize,
    field: usize,
    target: TranslationTarget,
    text: String,
}

// read title and text & translates it
pub fn process(timeline_data: *mut api::Il2CppObject) {
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

    let mut pending: Vec<PendingBlock> = Vec::new();
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

        let mut text_ptr: *mut api::Il2CppString = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(clip as *mut api::Il2CppObject, text_field(), &mut text_ptr as *mut _ as *mut c_void);
        }

        let text = unsafe { read_il2cpp_string(text_ptr) };
        let needs_tl = needs_translation(&text);
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
            let choice_needs_tl = needs_translation(&choice_text);
            crate::logging::info(&format!("story::process: block {i} choice {j} text = {choice_text:?} (needs_translation={choice_needs_tl})"));

            if choice_needs_tl {
                pending.push(PendingBlock {
                    index: i as usize,
                    obj: choice_obj as usize,
                    field: choice_text_field() as usize,
                    target: TranslationTarget::Choice(j as usize),
                    text: choice_text.clone(),
                });
            }
            choice_texts.push(choice_text);
        }

        if needs_tl {
            // text: none until translated, update_and_save_dict fills this in later
            block_dicts.push(cache::BlockDict { choice_data_list: choice_texts, ..Default::default() });
            pending.push(PendingBlock {
                index: i as usize,
                obj: clip as usize,
                field: text_field() as usize,
                target: TranslationTarget::Text,
                text,
            });
        } else {
            block_dicts.push(cache::BlockDict { text: Some(text), choice_data_list: choice_texts, ..Default::default() });
        }
    }

    {
        let mut guard = CURRENT_DICT.lock().unwrap();
        *guard = Some(CurrentDict {
            story_name,
            dict: cache::Dict {
                title: if title.is_empty() { None } else { Some(title) },
                text_block_list: block_dicts,
                no_wrap: false,
            },
        });
        if let Some(current) = guard.as_ref() {
            cache::save_dict(&current.story_name, &current.dict);
        }
    }

    if pending.is_empty() {
        return;
    }

    // attempt to prevent a race condition with the first tranlsated text
    let first = pending.remove(0);
    if let Some(translated) = llm::translate(&first.text) {
        let wrapped = wrap_text(&translated, 21);
        crate::logging::info(&format!("story::process: first block translated = {wrapped:?}"));
        write_translated_text(first.obj as *mut api::Il2CppObject, first.field as *mut FieldInfo, &wrapped);
        update_and_save_dict(first.index, &first.target, &wrapped);
    }

    if pending.is_empty() {
        return;
    }

    thread::spawn(move || {
        for block in pending {
            if let Some(translated) = llm::translate(&block.text) {
                let wrapped = wrap_text(&translated, 45);
                crate::logging::info(&format!("story::process: translated = {wrapped:?}"));

                PENDING_WRITES.lock().unwrap().push(PendingWrite { obj: block.obj, field: block.field, text: wrapped.clone() });
                update_and_save_dict(block.index, &block.target, &wrapped);

                unsafe {
                    (api::il2cpp_schedule_on_thread())(main_thread(), apply_pending_writes);
                }
            }
        }
    });
}