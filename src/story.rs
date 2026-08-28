use std::ffi::CString;
use std::sync::OnceLock;

use crate::api;
use crate::api::Il2CppClass;
use crate::api::FieldInfo;

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