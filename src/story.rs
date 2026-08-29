use std::ffi::CString;
use std::ffi::c_void;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::api;
use crate::api::Il2CppClass;
use crate::api::FieldInfo;
use crate::api::Il2CppObject;

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
    let story_timeline_controller_initialize = story_timeline_controller_initialize_addr();
    if story_timeline_controller_initialize.is_null() {
        crate::logging::warn("story::install: StoryTimelineController.Awake method not found");
    }
    else {
        crate::logging::info("story::install: StoryTimelineController.Awake method found");
    }
    install_story_timeline_controller_initialize_hook();
}

// boring resolution & hooking stuff
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

pub fn story_timeline_controller_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineController") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn story_timeline_controller_initialize_addr() -> *mut c_void {
    static METHOD: OnceLock<usize> = OnceLock::new();
    let addr = *METHOD.get_or_init(|| {
        let class = story_timeline_controller_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Initialize") else {return 0};
        unsafe { (api::il2cpp_get_method_addr())(class, name.as_ptr(), 0) as usize}
    });
    addr as *mut c_void
}

type StoryTimelineControllerAwake = unsafe extern "C" fn (
    this: *mut api::Il2CppObject
);

static TRAMPOLINE: OnceLock<usize> = OnceLock::new();
static PENDING_CONTROLLER: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn story_timeline_controller_initialize_hook (
    this: *mut api::Il2CppObject
) {
    if let Some(&trampoline) = TRAMPOLINE.get() {
        let orig: StoryTimelineControllerAwake = std::mem::transmute(trampoline);
        orig(this);
    }
    PENDING_CONTROLLER.store(this as usize, Ordering::Relaxed);
    crate::logging::info(&format!("story_timeline_controller_initialize_hook: fired"));
}

pub fn install_story_timeline_controller_initialize_hook() {
    let target = story_timeline_controller_initialize_addr();
    if target.is_null() {
        crate::logging::warn("install_story_timeline_controller_initialize_hook: Set_TimelineData address not found, skipping hook");
        return;
    }

    let trampoline = unsafe {
        let hachimi = (api::hachimi_instance())();
        let interceptor = (api::hachimi_get_interceptor())(hachimi);
        (api::interceptor_hook())(interceptor, target, story_timeline_controller_initialize_hook as *mut c_void)
    };

    if trampoline.is_null() {
        crate::logging::warn("install_story_timeline_controller_initialize_hook: interceptor_hook failed");
        return;
    }

    let _ = TRAMPOLINE.set(trampoline as usize);
    crate::logging::info("install_story_timeline_controller_initialize_hook: hook installed")

}

pub fn get_timeline_data_addr() -> *mut c_void {
    static METHOD: OnceLock<usize> = OnceLock::new();
    let addr = *METHOD.get_or_init( || {
        let class = story_timeline_controller_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("get_TimelineData") else {return 0};
        unsafe { (api::il2cpp_get_method_addr())(class, name.as_ptr(), 0) as usize}
    });
    addr as *mut c_void
}

type GetTimelineData = unsafe extern "C" fn (
    this: *mut api::Il2CppObject
) -> *mut api::Il2CppObject;

unsafe fn get_timeline_data(this: *mut api::Il2CppObject) -> *mut api::Il2CppObject {
    let addr = get_timeline_data_addr();
    if addr.is_null() {return std::ptr::null_mut();}
    let f: GetTimelineData = std::mem::transmute(addr);
    f(this)
}

pub fn poll() {
    let addr = PENDING_CONTROLLER.load(Ordering::Relaxed);
    if addr == 0 {
        return;
    }
    let this = addr as *mut api::Il2CppObject;
    let timeline_data = unsafe {get_timeline_data(this)};
    if timeline_data.is_null() {
        return; // nothing, check again next frame
    }
    PENDING_CONTROLLER.store(0, Ordering::Relaxed);
    crate::logging::info("story::poll: got TimelineData instance");

    let title = unsafe {read_title(timeline_data)} ; 
    crate::logging::info(&format!("story::poll: Title = {title:?}"))
}

// actual field reading logic
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

    if title_ptr.is_null() {
        return String::new();
    }

    unsafe {
        let len = (api::il2cpp_string_length())(title_ptr);
        if len <= 0 {
            return String::new();
        }
        let chars = (api::il2cpp_string_chars())(title_ptr);
        if chars.is_null() {
            return String::new();
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(chars, len as usize))
    }
}
