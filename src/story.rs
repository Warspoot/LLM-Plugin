use std::ffi::CString;
use std::ffi::c_void;
use std::sync::OnceLock;

use crate::api;
use crate::api::Il2CppClass;
use crate::api::FieldInfo;
use crate::il2cpp::{list_len, list_ref_at, read_il2cpp_string};

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

// check if text is translated
pub fn needs_translation(text: &str) -> bool {
    text.chars().any(|c| {
        let code = c as u32;
        (0x3040..=0x309F).contains(&code)   // Hiragana
            || (0x30A0..=0x30FF).contains(&code) // Katakana
            || (0x4E00..=0x9FFF).contains(&code) // Kanji
    })
}

/// read title and text
pub fn process(timeline_data: *mut api::Il2CppObject) {
    let title = read_title(timeline_data);
    crate::logging::info(&format!("story::process: Title = {title:?}"));

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

    for i in 0..count {
        let block = unsafe { list_ref_at(block_list_ptr, i) };
        if block.is_null() {
            continue;
        }

        let mut text_track: *mut c_void = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(block as *mut api::Il2CppObject, text_track_field(), &mut text_track as *mut _ as *mut c_void);
        }
        if text_track.is_null() {
            continue; // block has no dialogue
        }

        let mut clip_list_ptr: *mut c_void = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(text_track as *mut api::Il2CppObject, clip_list_field(), &mut clip_list_ptr as *mut _ as *mut c_void);
        }

        let clip = unsafe { list_ref_at(clip_list_ptr, 0) };
        if clip.is_null() {
            continue;
        }

        let mut text_ptr: *mut api::Il2CppString = std::ptr::null_mut();
        unsafe {
            (api::il2cpp_get_field_value())(clip as *mut api::Il2CppObject, text_field(), &mut text_ptr as *mut _ as *mut c_void);
        }

        let text = unsafe { read_il2cpp_string(text_ptr) };
        let needs_tl = needs_translation(&text);
        crate::logging::info(&format!("story::process: block {i} text = {text:?} (needs_translation={needs_tl})"));
    }
}